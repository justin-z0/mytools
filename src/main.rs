mod commands;
mod cli;

use clap::Parser;
use cli::YTSubCommand;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: YTSubCommand,
}


fn main() {
    let cli = Cli::parse();
    match &cli.command {
        YTSubCommand::Timestamp(cmd) => cmd.run(),
        YTSubCommand::Password(cmd) => cmd.run(),
    }
}


