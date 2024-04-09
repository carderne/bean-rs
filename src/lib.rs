//! # bean-rs
//!
//! `bean-rs` is a [beancount](https://github.com/beancount/beancount) clone (one day...) in Rust

pub mod book;
pub mod data;
pub mod error;
mod grammar;
pub mod ledger;
pub mod loader;
pub mod utils;

use pyo3::prelude::*;

use data::AccBal;

use crate::error::BeanError;
use crate::ledger::Ledger;

/// Loads the provided text into a Vec of Directives
/// containing opens, closes, transactions etc
pub fn load(text: String) -> Ledger {
    let entries = loader::load(&text);
    let ledger = loader::consume(entries);
    let mut dirs = ledger.dirs;
    loader::sort(&mut dirs);
    book::balance_transactions(&mut dirs);
    utils::debug_directives(&dirs);
    Ledger {
        dirs,
        errs: ledger.errs,
        opts: ledger.opts,
    }
}

/// Check and calculate balances for file at path
pub fn balance(path: &str) -> (AccBal, Vec<BeanError>) {
    let text = std::fs::read_to_string(path).expect("cannot read file");
    let mut ledger = load(text);
    let (bals, book_errs) = book::get_balances(&mut ledger.dirs);
    let mut errs = ledger.errs;
    errs.extend(book_errs);
    (bals, errs)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn py_balance(path: &str) -> PyResult<String> {
    balance(path);
    Ok("Ok".to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn bean_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_balance, m)?)?;
    Ok(())
}
