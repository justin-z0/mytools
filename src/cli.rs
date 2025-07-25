use clap::{Subcommand};
use crate::commands::{Runable, timestamp, password, completion};
use std::boxed::Box;

#[derive(Subcommand)]
pub enum YTCommand {
    /// 将时间戳转换为时区时间
    Timestamp(timestamp::TimestampCommand),
    /// 密码存取工具
    Password(password::PasswordCommand),
    /// 生成命令补全脚本
    Completion(completion::CompletionCommand),

    // 添加其他子命令
}

impl YTCommand {
    pub fn run(self) {
        let cmd: Box<dyn Runable> = match self {
            YTCommand::Timestamp(cmd) => Box::new(cmd),
            YTCommand::Password(cmd) => Box::new(cmd),
            YTCommand::Completion(cmd) => Box::new(cmd),
        };
        cmd.run();
    }
}