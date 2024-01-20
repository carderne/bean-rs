use crate::directives::{AccBal, Directive};
use std::collections::HashMap;

pub fn get_balances(directives: Vec<Directive>) -> AccBal {
    let mut bals: AccBal = HashMap::new();
    for d in directives {
        if let Directive::Transaction(tx) = d {
            for p in tx.postings {
                if let Some(amount) = p.amount {
                    let entry = bals.entry(p.account).or_default();
                    *entry.entry(amount.ccy.clone()).or_default() += amount.number;
                }
            }
        }
    }
    bals
}
