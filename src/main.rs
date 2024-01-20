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
    let cli = Cli::parse();
    match &cli.command {
        Commands::Balance { path } => {
            balance(path);
        }
    }
}

fn load(text: String) -> Vec<directives::Directive> {
    let entries = parser::parse(&text);
    let mut directives = parser::consume(entries);
    parser::sort(&mut directives);
    book::balance_transactions(&mut directives);
    utils::print_directives(&directives);
    directives
}

fn balance(path: &String) {
    let text = std::fs::read_to_string(path).expect("cannot read file");
    let directives = load(text);
    let bals = balance::get_balances(directives);
    utils::print_bals(bals);
    // println!("{bals:?}");
}
