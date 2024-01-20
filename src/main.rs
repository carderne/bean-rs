mod balance;
mod book;
mod directives;
mod grammar;
mod parser;
mod utils;

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

fn load(text: String) -> Result<Vec<directives::Directive>, parser::ParseError> {
    let entries = parser::parse(&text)?;
    let (dirs, bad) = parser::consume(entries);
    if !bad.is_empty() {
        utils::print_badlines(bad)
    }
    let mut dirs = dirs;
    parser::sort(&mut dirs);
    book::balance_transactions(&mut dirs);
    utils::print_directives(&dirs);
    Ok(dirs)
}

fn balance(path: &String) {
    let text = std::fs::read_to_string(path).expect("cannot read file");
    let directives = load(text).unwrap_or_else(|e| {
        eprintln!("Error: something went wrong: {e}");
        std::process::exit(1);
    });
    let bals = balance::get_balances(directives);
    utils::print_bals(bals);
}
