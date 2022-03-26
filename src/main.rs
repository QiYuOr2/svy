mod actions;
mod commands;

use crate::commands::Commands;
use clap::Parser;
// use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Registry(registry) => registry.exec(),
        Commands::Create(_) => {}
        Commands::Git(_) => {}
        Commands::Check => {}
    };
}
