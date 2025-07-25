mod commands;
mod cli;

use clap::Parser;
use cli::YTCommand;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: YTCommand,
}


fn main() {
    let cli = Cli::parse();
    cli.command.run();
}


