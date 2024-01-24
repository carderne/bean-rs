use bean_rs::balance;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Display account balances
    Balance { path: String },
    /// Check for errors and quit
    Check { path: String },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Balance { path } => {
            balance(path, true);
        }
        Commands::Check { path } => {
            balance(path, false);
        }
    }
}
