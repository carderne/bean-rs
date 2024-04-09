use std::collections::HashMap;

use log::debug;
use rust_decimal::Decimal;

use crate::{
    data::{AccBal, AccStatuses, Account, Amount, CcyBal, Directive, Pad, Posting, Transaction},
    error::{BeanError, ErrorType},
    loader,
};

/// Checks postings with no `Amount` and calculates the values
/// needed for the Transaction to balance.
///
/// If a Transaction is unbalanced but it has a Posting with no Amount,
/// then the account from that Posting is used to balance the transaction.
/// If there are multiple unbalanced currencies, a Posting will be
/// created for each one, all to the same Account.
fn complete_postings(tx: &mut Transaction) -> Vec<BeanError> {
    debug!("balancing {tx:?}");

    let mut ccy_bals: CcyBal = HashMap::new();

    // (Revisit this) Instead of appending to the original postings,
    // create a brand new Vec. Makes the logic of
    // deleting or adding 0, 1 or more Postings a bit easier
    let mut postings: Vec<Posting> = Vec::new();

    let mut errs: Vec<BeanError> = Vec::new();

    let mut found_empty_posting = false;
    let mut empty_posting_index = 0;

    for (i, p) in tx.postings.iter().enumerate() {
        match &p.amount {
            // if the posting has no amount specified
            None => {
                // there cant be more than one empty posting in a transaction
                if found_empty_posting {
                    let err = BeanError::new(
                        ErrorType::MultipleEmptyPostings,
                        &tx.debug,
                        "Found multiple empty postings for Transaction:",
                        Some(&Directive::Transaction(tx.clone())),
                    );
                    errs.push(err);
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
    errs
}

/// Checks that Transaction balances in all currencies to 0
/// MUST be run after `complete_postings`
fn check_transaction(tx: &Transaction) -> Vec<BeanError> {
    let mut errs: Vec<BeanError> = Vec::new();
    let mut ccy_bals: CcyBal = HashMap::new();
    for p in tx.postings.iter() {
        match &p.amount {
            // TODO use RawTransaction/Transaction and RawPosting/Posting to make impossible
            None => panic!("Found empty postings after they should have been replaced, abort."),
            Some(amount) => {
                *ccy_bals.entry(amount.ccy.clone()).or_default() += amount.number;
            }
        }
    }

    for (ccy, bal) in ccy_bals {
        // TODO get precision from context
        if bal.abs() > Decimal::new(1, 3) {
            let err = BeanError::new(
                ErrorType::UnbalancedTransaction,
                &tx.debug,
                &format!("Transaction unbalanced for currency: {ccy}"),
                Some(&Directive::Transaction(tx.clone())),
            );
            errs.push(err);
        }
    }
    errs
}

/// Complete postings as needed and check balances
/// Directives MUST be sorted appropriately before calling this
pub fn balance_transactions(directives: &mut [Directive]) -> Vec<BeanError> {
    let mut errs: Vec<BeanError> = Vec::new();
    for d in directives.iter_mut() {
        if let Directive::Transaction(tx) = d {
            errs.extend(complete_postings(tx));
            errs.extend(check_transaction(tx));
        }
    }
    errs
}

/// This is run within `get_balances`
/// Removed here as used in multiple places
fn proc_tx(tx: &Transaction, bals: &mut AccBal, accs: &mut AccStatuses, errs: &mut Vec<BeanError>) {
    for p in &tx.postings {
        if let Some(amount) = &p.amount {
            let status = accs.get(&p.account);
            match status {
                Some(open) => {
                    // the account is open: this is the happy path!
                    if open.0 {
                        let ccy = p.amount.clone().unwrap().ccy;
                        if open.1.is_empty() || open.1.contains(&ccy) {
                            let entry = bals.entry(p.account.clone()).or_default();
                            *entry.entry(amount.ccy.clone()).or_default() += amount.number;
                        } else {
                            let err = BeanError::new(
                                ErrorType::InvalidCcy,
                                &tx.debug,
                                &format!(
                                    "Invalid currency {ccy} for {account}",
                                    account = &p.account
                                ),
                                Some(&Directive::Transaction(tx.clone())),
                            );
                            errs.push(err);
                        }
                    // the account has been closed
                    } else {
                        let err = BeanError::new(
                            ErrorType::ClosedAccount,
                            &tx.debug,
                            &format!(
                                "Transaction referred to closed Account: {account}",
                                account = &p.account
                            ),
                            Some(&Directive::Transaction(tx.clone())),
                        );
                        errs.push(err);
                    }
                }
                // the account was never opened at all (it doesnt exist)
                None => {
                    let err = BeanError::new(
                        ErrorType::NoAccount,
                        &tx.debug,
                        &format!(
                            "Transaction referred to non-existent Account: {account}",
                            account = &p.account
                        ),
                        Some(&Directive::Transaction(tx.clone())),
                    );
                    errs.push(err);
                }
            }
        }
    }
}

/// Get balances for all accounts in all currencies
pub fn get_balances(dirs: &mut Vec<Directive>) -> (AccBal, Vec<BeanError>) {
    let mut bals: AccBal = HashMap::new();
    let mut accs: AccStatuses = HashMap::new();
    let mut errs: Vec<BeanError> = Vec::new();
    let mut pads: HashMap<Account, (bool, Pad)> = HashMap::new();
    let mut ptxs: Vec<Directive> = Vec::new();

    for d in dirs.iter() {
        match d {
            Directive::Open(open) => {
                if let Some(opened) = accs.get(&open.account) {
                    // the account has already been opened
                    if opened.0 {
                        let err = BeanError::new(
                            ErrorType::DuplicateOpen,
                            &open.debug,
                            &format!(
                                "Duplicate open directive for {account}",
                                account = &open.account
                            ),
                            Some(&Directive::Open(open.clone())),
                        );
                        errs.push(err);
                    }
                }
                let valid_ccys = open.ccys.clone();
                accs.insert(open.account.clone(), (true, valid_ccys));
            }
            Directive::Close(close) => {
                if let Some(opened) = accs.get(&close.account) {
                    // the account has already been closed
                    if !opened.0 {
                        let err = BeanError::new(
                            ErrorType::DuplicateClose,
                            &close.debug,
                            &format!(
                                "Duplicate close directive for {account}",
                                account = &close.account
                            ),
                            Some(&Directive::Close(close.clone())),
                        );
                        errs.push(err);
                    }
                }
                accs.insert(close.account.clone(), (false, Vec::new()));
            }
            Directive::Pad(pad) => {
                let acc = &pad.account_to;
                if let Some(val) = pads.get(acc) {
                    let (used, prev_pad) = val;
                    if !used {
                        let err = BeanError::new(
                            ErrorType::UnusedPad,
                            &prev_pad.debug,
                            &format!("Multiple pads for {acc}"),
                            Some(&Directive::Pad(prev_pad.clone())),
                        );
                        errs.push(err);
                    }
                }
                pads.insert(acc.clone(), (false, pad.clone()));
            }
            Directive::Balance(bal) => {
                // Check the Balance directive against the accumulated balance in `bals`

                // Get the accumulated balance
                let def = &Decimal::default();
                let ccy = &bal.amount.ccy;
                let entry = bals.entry(bal.account.clone()).or_default();

                // Compare against the current balance
                let accum_bal = entry.get(ccy).unwrap_or(def);
                let assert_bal = bal.amount.number;
                let diff = assert_bal - *accum_bal;
                // TODO get precision from context
                if diff > Decimal::new(1, 3) {
                    // If we have a Pad available to use to make up the difference
                    if let Some(val) = pads.get(&bal.account) {
                        let (_, pad) = val;
                        // The amount is the diff and we use the ccy from the Balance
                        // as the Pad has no ccy
                        let amount = Amount::new(diff, ccy.clone());

                        // Create a new Transaction that will stand in for the Pad
                        let newtx = Transaction::from_pad(pad.clone(), amount);

                        // Keep track that the Pad has been 'used'
                        // Can be used again for another ccy if needed
                        pads.insert(bal.account.clone(), (true, pad.clone()));

                        // Process the new transaction
                        proc_tx(&newtx, &mut bals, &mut accs, &mut errs);
                        // And then add it to the main Vec of directives
                        let newtx = Directive::Transaction(newtx);
                        ptxs.push(newtx);
                    } else {
                        let err = BeanError::new(
                            ErrorType::BalanceAssertion,
                            &bal.debug,
                            &format!("Balance assertion failed: asserted {assert_bal} is not equal to {accum_bal}"),
                            Some(&Directive::Balance(bal.clone())),
                        );
                        errs.push(err);
                    }
                }
                // If the balance is fine, do nothing!
            }
            Directive::Transaction(tx) => {
                proc_tx(tx, &mut bals, &mut accs, &mut errs);
            }
            _ => (),
        }
    }
    dirs.extend(ptxs);
    loader::sort(dirs);
    (bals, errs)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::data::{DebugLine, DATE_FMT};

    use super::*;

    #[test]
    fn test_bad_transaction() {
        let p1 = Posting {
            account: "Assets:Bank".to_string(),
            amount: None,
            debug: None,
        };
        let p2 = p1.clone();
        let date = NaiveDate::parse_from_str("2023-01-01", DATE_FMT).unwrap();
        let mut tx = Transaction {
            date,
            ty: "*".to_string(),
            payee: None,
            narration: "".to_string(),
            tag: None,
            link: None,
            postings: vec![p1, p2],
            meta: vec![],
            debug: DebugLine { line: 0 },
        };
        let errs = complete_postings(&mut tx);
        assert!(errs.first().unwrap().ty == ErrorType::MultipleEmptyPostings);
    }

    #[test]
    fn test_bad_ccy() {
        let p1 = Posting {
            account: "Assets:Bank".to_string(),
            amount: Some(Amount::new(Decimal::new(100, 1), "USD".to_string())),
            debug: None,
        };
        let p2 = Posting {
            account: "Income:Job".to_string(),
            amount: Some(Amount::new(Decimal::new(-100, 1), "USD".to_string())),
            debug: None,
        };
        let date = NaiveDate::parse_from_str("2023-01-01", DATE_FMT).unwrap();
        let tx = Transaction {
            date,
            ty: "*".to_string(),
            payee: None,
            narration: "".to_string(),
            tag: None,
            link: None,
            postings: vec![p1, p2],
            meta: vec![],
            debug: DebugLine { line: 0 },
        };
        let mut bals: AccBal = HashMap::new();
        let mut accs: AccStatuses = HashMap::new();
        accs.insert("Assets:Bank".to_string(), (true, vec!["FOO".to_string()]));
        accs.insert("Income:Job".to_string(), (true, vec!["FOO".to_string()]));
        let mut errs: Vec<BeanError> = vec![];
        proc_tx(&tx, &mut bals, &mut accs, &mut errs);
        assert!(errs.first().unwrap().ty == ErrorType::InvalidCcy);
    }

    #[test]
    fn test_closed_acc() {
        let p1 = Posting {
            account: "Assets:Bank".to_string(),
            amount: Some(Amount::new(Decimal::new(100, 1), "USD".to_string())),
            debug: None,
        };
        let p2 = Posting {
            account: "Income:Job".to_string(),
            amount: Some(Amount::new(Decimal::new(-100, 1), "USD".to_string())),
            debug: None,
        };
        let date = NaiveDate::parse_from_str("2023-01-01", DATE_FMT).unwrap();
        let tx = Transaction {
            date,
            ty: "*".to_string(),
            payee: None,
            narration: "".to_string(),
            tag: None,
            link: None,
            postings: vec![p1, p2],
            meta: vec![],
            debug: DebugLine { line: 0 },
        };
        let mut bals: AccBal = HashMap::new();
        let mut accs: AccStatuses = HashMap::new();
        accs.insert("Assets:Bank".to_string(), (false, vec!["USD".to_string()]));
        accs.insert("Income:Job".to_string(), (false, vec!["USD".to_string()]));
        let mut errs: Vec<BeanError> = vec![];
        proc_tx(&tx, &mut bals, &mut accs, &mut errs);
        assert!(errs.first().unwrap().ty == ErrorType::ClosedAccount);
    }
}
