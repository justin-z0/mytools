use clap::{Subcommand};
use crate::commands::*;

#[derive(Subcommand)]
pub enum YTSubCommand {
    Timestamp(timestamp::TimestampCommand),
    Password(password::PasswordCommand),
    // 添加其他子命令
}