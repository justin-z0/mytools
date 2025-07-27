use clap::Parser;
use crossterm::{
    cursor,
    execute,
    event::{Event, KeyCode, read},
    terminal::{self, ClearType, enable_raw_mode, disable_raw_mode},
};
use std::io::{stdout, Write};

#[derive(Parser)]
#[command(about="模拟彩票机选下注，输入：\n    Enter: 确认\n    Space: 重选")]
pub struct LotteryCommand {
    /// 下注数量
    ///
    #[arg(default_value_t = 1)]
    bets: u8,
}

struct Lottery {
    red: [u8; 6],
    blue: u8,
}

impl super::Runable for LotteryCommand {
    fn run(&self) {
        println!("机选: {} 注", self.bets);
        loop {
            for _ in 0..self.bets {
                generate_lottery_number();
            }

            if let Ok(false) = will_continue(self.bets as u16) {
                break;
            }
        }

        stdout().flush().unwrap();
    }
}

fn generate_lottery_number() {
    let mut lottery = Lottery {
        red: [0; 6],
        blue: 0,
    };

    for i in 0..lottery.red.len() {
        lottery.red[i] = rand::random::<u8>() % 33 + 1;
    }
    lottery.blue = rand::random::<u8>() % 16 + 1;

    // 格式化红色球数字，每个占两位，不足补0，空格分隔
    let red_str: Vec<String> = lottery
        .red
        .iter()
        .map(|&num| format!("{:02}", num))
        .collect();
    println!("{}    {:02}", red_str.join(" "), lottery.blue);
}

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
    ).unwrap();

    // 使用 cursor 移动了光标后，需要刷新一次缓存，否则后续显示会错乱
    stdout().flush().unwrap();

    Ok(true)
}
