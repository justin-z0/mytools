use crate::commands::*;
use clap::Subcommand;
use std::boxed::Box;

/*
* 对子命令添加描述信息：
* 方式1： 使用 /// 注释，此种方式描述只能在一行
* 方式2： 在子命令定义的地方使用 #[command(about="")] 宏定义，此种方式支持转义符号
*/

#[derive(Subcommand)]
pub enum MTCommand {
    /// 将时间戳转换为时区时间
    Timestamp(timestamp::TimestampCommand),
    /// 密码存取工具
    Password(password::PasswordCommand),
    /// 生成命令补全脚本
    Completion(completion::CompletionCommand),
    // 彩票号码生成工具
    Lottery(lottery::LotteryCommand),
    // 添加其他子命令
}

impl MTCommand {
    pub fn run(self) {
        let cmd: Box<dyn Runable> = match self {
            MTCommand::Timestamp(cmd) => Box::new(cmd),
            MTCommand::Password(cmd) => Box::new(cmd),
            MTCommand::Completion(cmd) => Box::new(cmd),
            MTCommand::Lottery(cmd) => Box::new(cmd),
        };
        cmd.run();
    }
}
