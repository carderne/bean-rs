mod directives;
mod grammar;
mod parser;
mod utils;

use clap::{Parser, Subcommand};

use crate::utils::print_directives;

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
    let cli = Cli::parse();
    match &cli.command {
        Commands::Balance { path } => {
            balance(path);
        }
    }
}

fn balance(path: &String) {
    let text = std::fs::read_to_string(path).expect("cannot read file");
    let entries = parser::parse(&text);
    let directives = parser::consume(entries);
    print_directives(directives);
}
