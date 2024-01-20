use clap::{Parser, Subcommand};
use bean_rs::{balance};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Balance { path: String },
}

fn main() {
    env_logger::init();
    let cli = Cli::parse();
    match &cli.command {
        Commands::Balance { path } => {
            balance(path);
        }
    }
}
