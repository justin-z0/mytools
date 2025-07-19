use chrono::*;
use clap::Parser;

#[derive(Parser, Debug)]
pub struct TimestampCommand {
    /// 命令行的输入
    input: String,

    /// 是否为毫秒
    #[arg(short, long, action=clap::ArgAction::SetTrue)]
    ms: bool
}

impl TimestampCommand {
    pub fn run(&self) {
        println!("执行命令：{:?}", self);
        let output = format_timestamp(&self.input);
        println!("处理结果为：{}", output);
    }
}

/// 格式化时间戳为可读格式
/// # 参数
/// * `timestamp` - 要格式化的时间戳字符串
/// # 返回
/// 格式化后的时间字符串，如果出错则返回错误信息
pub fn format_timestamp(timestamp: &String) -> String {
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
