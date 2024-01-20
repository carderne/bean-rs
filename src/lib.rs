//! # bean-rs
//!
//! `bean-rs` is a [beancount](https://github.com/beancount/beancount) clone (one day...) in Rust

mod balance;
mod book;
mod directives;
mod grammar;
mod parser;
mod utils;

/// Loads the provided text into a Vec of Directives
/// containing opens, closes, transactions etc
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

/// Load the file at `path` and print the balance
pub fn balance(path: &String) {
    let text = std::fs::read_to_string(path).expect("cannot read file");
    let directives = load(text).unwrap_or_else(|e| {
        eprintln!("Error: something went wrong: {e}");
        std::process::exit(1);
    });
    let bals = balance::get_balances(directives);
    utils::print_bals(bals);
}
