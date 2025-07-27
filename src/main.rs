mod commands;
mod cli;

use clap::Parser;
use cli::YTCommand;

#[derive(Parser)]
#[command(version, about="Your Tools")]
struct Cli {
    #[command(subcommand)]
    command: YTCommand,
}


fn main() {
    let cli = Cli::parse();
    cli.command.run();
}


