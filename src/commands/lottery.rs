use anyhow::{Result, anyhow};
use chrono::{Datelike, NaiveDate};
use clap::{Parser, Subcommand};
use crossterm::{
    cursor,
    event::{Event, KeyCode, read},
    execute,
    terminal::{self, ClearType, disable_raw_mode, enable_raw_mode},
};
use regex::Regex;
use reqwest::blocking::Client;
use rusqlite::Connection;
use std::{
    io::{Write, stdout},
    str::FromStr,
};
use dirs::home_dir;

#[derive(Parser)]
#[command(about = "模拟彩票机选下注，输入：\n    Enter: 确认\n    Space: 重选")]
pub struct LotteryCommand {
    /// 下注数量
    #[arg(default_value_t = 1)]
    bets: u8,
    #[command(subcommand)]
    cmd: Option<CheckCommand>,
}

#[derive(Subcommand)]
enum CheckCommand {
    /// 检查是否中奖
    Check,
}
struct Lottery {
    red: [u8; 6],
    blue: u8,
}

impl FromStr for Lottery {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 7 {
            return Err(anyhow!("Invalid lottery format"));
        }
        let red = parts[0..6].iter().map(|x| x.parse().unwrap()).collect::<Vec<u8>>();
        let blue = parts[6].parse().unwrap();
        Ok(Lottery { red: red.try_into().unwrap(), blue })
    }
}
/// 开奖信息
struct KaiJiang {
    /// 开奖时间
    open_time: String,
    /// 开奖号码
    kjhm: String,
}

struct Record {
    // 期号
    issue: String,
    // 开奖时间
    open_time: String,
    // 开奖号码
    kjhm: String,
    // 投注号码，为了支持多注，使用双引号包裹，逗号分割
    tzhm: String,
    // 中奖等级，0 表示未中奖
    level: u8,
}

use std::fmt;
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tzhm = self.tzhm.split(';')
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join("\n          ");
        write!(f, "    期号: {}\n开奖时间: {}\n开奖号码: {}\n投注号码: {}\n中奖等级: {}",
               self.issue, self.open_time, self.kjhm, tzhm, self.level)
    }
}

impl super::Runable for LotteryCommand {
    fn run(&self) {
        // 读取历史数据
        let mut data_path = home_dir().expect("无法访问Home目录");
        data_path.push(".yt/lottery/data.db");

        // 确保目录存在
        if let Some(parent) = data_path.parent() {
            std::fs::create_dir_all(parent).expect("无法创建数据目录");
        }

        let conn = Connection::open(data_path).unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS record (issue TEXT PRIMARY KEY, open_time TEXT, kjhm TEXT, tzhm TEXT, level INTEGER)", ()).unwrap();

        match &self.cmd {
            Some(CheckCommand::Check) => {
                // 处理 Check 子命令的逻辑
                // 1. 从数据库中读取最后一期自选记录
                if let Some(mut record) = get_last_record(&conn).unwrap() {
                    // 2. 从网站中读取下注期号的开奖信息
                    match fetch_kaijiang_info(&record.issue) {
                        Ok(info) => {
                            let need_update = record.open_time.is_empty();
                            record.open_time = info.open_time.clone();
                            record.kjhm = info.kjhm.clone();
                            let tzhms: Vec<Lottery> = record.tzhm.split(';')
                                                        .map(|x| x.parse().unwrap())
                                                        .collect();
                            let kjhm: Lottery = info.kjhm.parse().unwrap();
                            record.level = validate_lottery(&kjhm, &tzhms);
                            println!("{}", record);
                            // 3. 将信息写入数据库
                            if need_update {
                                conn.execute("UPDATE record set open_time=?, kjhm=?, level=? WHERE issue=?",
                                    (record.open_time, record.kjhm, record.level, record.issue)).unwrap();
                            }
                        }
                        Err(_) => {
                            println!("获取开奖信息失败：\n\n{}", record);
                        }
                    }
                }
            }
            None => {
                // 没有子命令时的默认行为
                let cur_issue = get_cur_issue().unwrap();
                println!("  期号: {}", cur_issue);
                println!("双色球: {} 注", self.bets);
                let mut bets: Vec<Lottery> = Vec::new();
                loop {
                    for _ in 0..self.bets {
                        bets.push(generate_lottery_number());
                    }
                    match will_continue(self.bets as u16) {
                        Ok(true) => {
                            bets.clear();
                            continue;
                        }
                        _ => {
                            break;
                        }
                    }
                }
                write_bets(&bets, &conn, &cur_issue).unwrap();
                stdout().flush().unwrap();
            }
        }
    }
}

fn generate_lottery_number() -> Lottery {
    let mut lottery = Lottery {
        red: [0; 6],
        blue: 0,
    };

    for i in 0..lottery.red.len() {
        let mut n: u8;
        loop {
            n = rand::random::<u8>() % 33 + 1;
            if !lottery.red.contains(&n) {
                break;
            }
        }
        lottery.red[i] = n;
    }
    lottery.red.sort();
    lottery.blue = rand::random::<u8>() % 16 + 1;

    // 格式化红色球数字，每个占两位，不足补0，空格分隔
    let red_str: Vec<String> = lottery
        .red
        .iter()
        .map(|&num| format!("{:02}", num))
        .collect();
    println!("{}    {:02}", red_str.join(" "), lottery.blue);

    lottery
}

// 根据输入判断是否继续。仅在输入 Enter 时才会退出
fn will_continue(bets: u16) -> Result<bool, std::io::Error> {
    // 进入 raw 模式，输入不会被缓存，确保 read 能够及时督导键盘输入
    enable_raw_mode()?;
    if let Event::Key(key) = read()? {
        if key.code == KeyCode::Enter {
            disable_raw_mode()?;
            return Ok(false);
        }
    }
    disable_raw_mode()?;

    // 清理屏幕，重新生成
    execute!(
        stdout(),
        cursor::MoveToPreviousLine(bets),
        terminal::Clear(ClearType::FromCursorDown)
    )
    .unwrap();

    // 使用 cursor 移动了光标后，需要刷新一次缓存，否则后续显示会错乱
    stdout().flush().unwrap();

    Ok(true)
}

// 抓取当前彩票开奖信息
fn fetch_kaijiang_info(issue: &String) -> Result<KaiJiang> {
    // 从文件中获取期数
    let base_uri = "https://jc.zhcw.com/port/client_json.php";
    let url = format!("{base_uri}");

    let client = Client::new();
    let query = [
        ("callback", "jQuery112209650134855416691_1754136840385"),
        ("transactionType", "10001002"),
        ("lotteryId", "1"),
        ("issue", issue.as_str()),
    ];
    let response = client.get(url)
        .query(&query)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .header("Referer", "https://www.zhcw.com/")
        .send()?;

    if response.status() == 200 {
        let body = response.text()?;
        let re = Regex::new(
            r#""issue":"(.*?)".*"openTime":"(.*?)".*"frontWinningNum":"(.*?)".*"backWinningNum":"(.*?)""#,
        )?;
        if let Some(caps) = re.captures(&body) {
            let mut kjhm = caps[3].to_string() + " " + &caps[4];
            kjhm = kjhm.replace(' ', ",");
            return Ok(KaiJiang {
                open_time: caps[2].to_string(),
                kjhm, // red + blue
            });
        }
    }

    return Err(anyhow!("获取彩票开奖信息失败"));
}

/// 获取当前期号
/// 先获取最后一次的开奖期号，然后根据时间判断下一次的期号(年末，期号需要跳变)
fn get_cur_issue() -> Result<String> {
    let url = "https://jc.zhcw.com/port/client_json.php";
    let client = Client::new();
    let query = [
        ("callback", "jQuery112209650134855416691_1754136840385"),
        ("transactionType", "10001001"),
        ("lotteryId", "1"),
        ("issueCount", "1"),
        ("type", "0"),
        ("pageNum", "1"),
        ("pageSize", "30"),
    ];
    let response = client.get(url)
        .query(&query)
        .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")
        .header("Referer", "https://www.zhcw.com/")
        .send()?;

    let body = response.text()?;
    let re = Regex::new(r#""issue":"(.*?)".*"openTime":"(.*?)".*"week":"(.*?)""#)?;
    if let Some(caps) = re.captures(&body) {
        let week = &caps[3].chars().last().unwrap();
        let step = match week {
            '四' => 3,
            _ => 2,
        };

        let mut issue = u32::from_str_radix(&caps[1], 10)?;

        let open_time = NaiveDate::parse_from_str(&caps[2], "%Y-%m-%d")?;
        if open_time.month() == 12 && open_time.day() + step > 31 {
            // 一年边间，需要跳变
            return Ok(format!("{}001", open_time.year() + 1));
        } else {
            issue += 1;
            return Ok(issue.to_string());
        }
    }

    Err(anyhow!("获取当前期号失败"))
}

fn write_bets(bets: &Vec<Lottery>, conn: &Connection, issue: &String) -> Result<()> {
    // 将 bets 写入数据库
    let mut tzhm = String::new();
    for bet in bets {
        for red in bet.red {
            tzhm.push_str(&format!("{:02},", red));
        }
        tzhm.push_str(&format!("{:02};", bet.blue));
    }

    let mut sql = conn.prepare("SELECT tzhm from record where issue=?1")?;
    let record = sql.query_row([issue], |row| row.get::<_, String>(0));

    if let Ok(result) = record {
        tzhm.push_str(&result);
    }

    sql = conn.prepare(
        "INSERT INTO record (issue, tzhm) VALUES (?1, ?2)
                                ON CONFLICT(issue) DO UPDATE SET tzhm = ?2",
    )?;
    tzhm =tzhm.trim_end_matches(";").to_string();
    sql.execute((issue, &tzhm))?;
    Ok(())
}

// 在 lottery.rs 文件中添加获取最后一条记录的函数

/// 获取数据库中最后一期开奖记录
fn get_last_record(conn: &Connection) -> Result<Option<Record>> {
    let mut stmt = conn.prepare(
        "SELECT issue, open_time, kjhm, tzhm, level FROM record \
         ORDER BY issue DESC LIMIT 1",
    )?;

    let record = stmt.query_row([], |row| {
        Ok(Record {
            issue: row.get(0)?,
            open_time: row.get(1).unwrap_or_default(),
            kjhm: row.get(2).unwrap_or_default(),
            tzhm: row.get(3)?,
            level: row.get(4).unwrap_or_default(),
        })
    });

    match record {
        Ok(r) => Ok(Some(r)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

// 一等奖：6红 + 1蓝
// 二等奖：6红
// 三等奖：5红 + 1蓝
// 四等奖：5红 或者 4红 + 1蓝
// 五等奖：4红 或者 3红 + 1蓝
// 六等奖：1蓝
fn validate_lottery(kjhm: &Lottery, tzhm: &Vec<Lottery>) -> u8 {
    let mut max_level = 7;
    for bet in tzhm {
        let level;
        let hit_blue = bet.blue == kjhm.blue;
        match bet.red.iter().filter(|&x| kjhm.red.contains(x)).count() {
            6 => if hit_blue { level = 1 } else { level = 2 },
            5 => if hit_blue { level = 3 } else { level = 4 },
            4 => if hit_blue { level = 4 } else { level = 5 },
            3 => if hit_blue { level = 5 } else { level = 0 },
            _ => if hit_blue { level = 6 } else { level = 0 }
        };

        if level < max_level {
            max_level = level;
        }
    }

    if max_level < 7 { max_level } else { 0 }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lottery_from_str() {
        let lottery_str = "01,02,03,04,05,06;07";
        let lottery: Lottery = lottery_str.parse().unwrap();

        assert_eq!(lottery.red, [1, 2, 3, 4, 5, 6]);
        assert_eq!(lottery.blue, 7);
    }

    #[test]
    fn test_lottery_from_str_invalid_format() {
        let lottery_str = "01,02,03,04,05;06"; // Only 5 red balls
        let result: Result<Lottery, _> = lottery_str.parse();
        assert!(result.is_err());
    }
}
