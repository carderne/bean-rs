use std::collections::HashMap;

use crate::directives::{CcyBal, Directive, Posting, Transaction};

fn balance_transaction(tx: &mut Transaction) {
    eprintln!("balancing {tx:?}");

    let mut ccy_bals: CcyBal = HashMap::new();
    let mut postings: Vec<Posting> = Vec::new();

    let mut found_empty_posting = false;
    let mut empty_posting_index = 0;

    for (i, p) in tx.postings.iter().enumerate() {
        match &p.amount {
            None => {
                if found_empty_posting {
                    panic!("cannot have multiple empty postings")
                }
                empty_posting_index = i;
                found_empty_posting = true;
            }
            Some(amount) => {
                if let Some(value) = ccy_bals.get(&amount.ccy) {
                    ccy_bals.insert(amount.ccy.clone(), value + amount.number);
                } else {
                    ccy_bals.insert(amount.ccy.clone(), amount.number);
                }
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
}

pub fn balance_transactions(directives: &mut [Directive]) {
    for d in directives.iter_mut() {
        if let Directive::Transaction(tx) = d {
            balance_transaction(tx);
        }
    }
}
