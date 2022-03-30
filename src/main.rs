mod actions;
mod commands;
mod common;

use clap::Parser;
use commands::Commands;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    let config = common::check_config();

    match config {
        Ok(mut svy) => {
            match &cli.command {
                Commands::Registry(registry) => registry.exec(&mut svy),
                Commands::Create(_) => {}
                Commands::Git(_) => {}
                Commands::Check { keywords } => actions::check(keywords).unwrap(),
            };
        }
        Err(_) => {}
    }
}
