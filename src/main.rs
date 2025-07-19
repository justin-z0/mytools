use chrono::*;
use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 时间戳格式化为可读的格式
    /// 用法: yt fts <timestamp>
    Fts {
        /// 要格式化的时间戳
        timestamp: String,

        /// 指定输入为毫秒格式
        #[arg(short, long)]
        ms: Option<String>,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Some(Commands::Fts { timestamp, ms: _}) => {
            let result = format_timestamp(timestamp);
            println!("格式化时间：{}", result);
        }
        None => {
            println!("请输入命令");
        }
    }
}

/// 格式化时间戳为可读格式
/// # 参数
/// * `timestamp` - 要格式化的时间戳字符串
/// # 返回
/// 格式化后的时间字符串，如果出错则返回错误信息
fn format_timestamp(timestamp: String) -> String {
    match timestamp.parse::<i64>() {
        Ok(ts) => {
            // 使用chrono格式化时间戳
            let datetime = DateTime::<Utc>::from_timestamp(ts, 0);
            match datetime {
                Some(dt) => {
                    let formatted = dt.format("%Y-%m-%d %H:%M:%S").to_string();
                    formatted
                }
                None => {
                    let error_msg = "错误: 无效的时间戳".to_string();
                    println!("{}", error_msg);
                    error_msg
                }
            }
        }
        Err(_) => {
            let error_msg = "错误: 时间戳格式无效，请输入有效的Unix时间戳".to_string();
            println!("{}", error_msg);
            error_msg
        }
    }
}
