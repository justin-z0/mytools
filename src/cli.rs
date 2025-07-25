use clap::{Subcommand};
use crate::commands::{Runable, timestamp, password};
use std::boxed::Box;

#[derive(Subcommand)]
pub enum YTSubCommand {
    Timestamp(timestamp::TimestampCommand),
    Password(password::PasswordCommand),
    // 添加其他子命令
}

impl YTSubCommand {
    pub fn run(self) {
        let cmd: Box<dyn Runable> = match self {
            YTSubCommand::Timestamp(cmd) => Box::new(cmd),
            YTSubCommand::Password(cmd) => Box::new(cmd),
        };
        cmd.run();
    }
}