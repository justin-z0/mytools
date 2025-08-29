mod cli;
mod commands;

use clap::Parser;
use cli::MTCommand;

#[derive(Parser)]
#[command(version, about = "My Tools")]
struct Cli {
    #[command(subcommand)]
    command: MTCommand,
}

fn main() {
    let cli = Cli::parse();
    cli.command.run();
}
