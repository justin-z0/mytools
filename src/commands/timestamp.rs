use chrono::{Utc, DateTime};
use clap::Parser;
use chrono_tz::Asia::Shanghai;

#[derive(Parser, Debug)]
#[command(version)]
pub struct TimestampCommand {
    /// 当不指定时，用于获取系统当前时间戳
    #[arg(default_value = "")]
    input: String,

    /// 是否为毫秒
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    ms: bool
}

impl super::Runable for TimestampCommand {
    fn run(&self) {
        if self.input.is_empty() {
            let timestamp = get_timestamp(self.ms);
            println!("当前时间戳为: {}", timestamp);
        } else {
            let formated_time = format_timestamp(&self.input, self.ms);
            println!("格式化后的时间为: {}", formated_time);
        }
    }
}

/// 格式化时间戳为可读格式
/// # 参数
/// * `timestamp` - 要格式化的时间戳字符串
/// # 返回
/// 格式化后的时间字符串，如果出错则返回错误信息
fn format_timestamp(timestamp: &String, is_ms: bool) -> String {
        match timestamp.parse::<i64>() {
            Ok(ts) => {
                let datetime = if is_ms {
                    // 毫秒级时间戳
                    DateTime::<Utc>::from_timestamp_millis(ts)
                } else {
                    // 秒级时间戳
                    DateTime::<Utc>::from_timestamp(ts, 0)
                }.unwrap();

                datetime.with_timezone(&Shanghai)
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string()
            }
            Err(_) => {
                let error_msg = "错误: 时间戳格式无效，请输入有效的Unix时间戳".to_string();
                error_msg
            }
        }
}

fn get_timestamp(is_ms: bool) -> u64 {
    if is_ms {
        Utc::now().timestamp_millis() as u64
    } else {
        Utc::now().timestamp() as u64
    }
}