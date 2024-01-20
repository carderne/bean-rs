use std::{collections::HashMap, str::FromStr};
use std::fmt;

use log::debug;
use rust_decimal::Decimal;

use crate::directives::{CcyBal, Directive, Posting, Transaction};
use crate::utils;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum BookErrorType {
    MultipleEmptyPostings,
    EmptyPosting,
    UnbalancedTransaction,
}
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BookError {
    ty: BookErrorType,
    line: usize,
    msg: String,
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "BookError: ({ty:?}) L{line}: {msg}",
            ty = self.ty,
            line = self.line,
            msg = self.msg,
        )
    }
}

/// Checks postings with no `Amount` and calculates the values
/// needed for the Transaction to balance.
fn complete_postings(tx: &mut Transaction) -> Vec<BookError> {
    debug!("balancing {tx:?}");

    let mut errors: Vec<BookError> = Vec::new();

    let mut ccy_bals: CcyBal = HashMap::new();
    let mut postings: Vec<Posting> = Vec::new();

    let mut found_empty_posting = false;
    let mut empty_posting_index = 0;

    for (i, p) in tx.postings.iter().enumerate() {
        match &p.amount {
            None => {
                if found_empty_posting {
                    let line = tx.debug.line;
                    let ty = BookErrorType::MultipleEmptyPostings;
                    let err = BookError {
                        ty,
                        line,
                        msg: String::from(""),
                    };
                    errors.push(err);
                }
                empty_posting_index = i;
                found_empty_posting = true;
            }
            Some(amount) => {
                *ccy_bals.entry(amount.ccy.clone()).or_default() += amount.number;
                postings.push(p.clone())
            }
        }
    }

    if found_empty_posting {
        let account = &tx.postings[empty_posting_index].account;
        for (ccy, number) in &ccy_bals {
            let p = Posting::new(account.clone(), -number, ccy.clone());
            postings.push(p.clone())
        }
    }

    tx.postings = postings;
    errors
}

/// Checks that Transaction balances in all currencies to 0
/// MUST be run after `complete_postings`
fn check_transaction(tx: &Transaction) -> Vec<BookError> {
    let mut errors: Vec<BookError> = Vec::new();
    let mut ccy_bals: CcyBal = HashMap::new();
    for p in tx.postings.iter() {
        match &p.amount {
            None => {
                let line = tx.debug.line;
                let ty = BookErrorType::EmptyPosting;
                let err = BookError {
                    ty,
                    line,
                    msg: String::from(""),
                };
                errors.push(err);
            }
            Some(amount) => {
                *ccy_bals.entry(amount.ccy.clone()).or_default() += amount.number;
            }
        }
    }

    for (ccy, bal) in ccy_bals {
        // TODO get precision from context
        if bal.abs() > Decimal::from_str("0.001").unwrap() {
            let line = tx.debug.line;
            let ty = BookErrorType::UnbalancedTransaction;
            let err = BookError {
                ty,
                line,
                msg: ccy.to_string(),
            };
            errors.push(err);
        }
    }
    errors
}

/// Complete postings as needed and check balances
pub fn balance_transactions(directives: &mut [Directive]) {
    for d in directives.iter_mut() {
        if let Directive::Transaction(tx) = d {
            let errs = complete_postings(tx);
            utils::print_book_errors(&errs);
            let errs = check_transaction(tx);
            utils::print_book_errors(&errs);
        }
    }
}
