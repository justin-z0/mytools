use clap::{Subcommand};
use crate::commands::*;

#[derive(Subcommand)]
pub enum YTSubCommand {
    Timestamp(timestamp::TimestampCommand),
}