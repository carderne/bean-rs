//! # bean-rs
//!
//! `bean-rs` is a [beancount](https://github.com/beancount/beancount) clone (one day...) in Rust

mod book;
mod directives;
pub mod error;
mod grammar;
mod parser;
pub mod utils;

use directives::AccBal;

use crate::directives::Directive;
use crate::error::BeanError;

/// Loads the provided text into a Vec of Directives
/// containing opens, closes, transactions etc
fn load(text: String) -> (Vec<Directive>, Vec<BeanError>) {
    let entries = parser::parse(&text);
    let entries = match entries {
        Ok(entries) => entries,
        Err(error) => {
            let empty_dirs: Vec<Directive> = Vec::new();
            return (empty_dirs, vec![error]);
        }
    };
    let (dirs, errs) = parser::consume(entries);
    let mut dirs = dirs;
    parser::sort(&mut dirs);
    book::balance_transactions(&mut dirs);
    utils::debug_directives(&dirs);
    (dirs, errs)
}

/// Check and calculate balances for file at path
pub fn balance(path: &String) -> (AccBal, Vec<BeanError>) {
    let text = std::fs::read_to_string(path).expect("cannot read file");
    let (mut dirs, mut errs) = load(text);
    let (bals, book_errs) = book::get_balances(&mut dirs);
    errs.extend(book_errs);
    (bals, errs)
}
