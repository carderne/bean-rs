use crate::directives::{AccBal, Directive};
use std::collections::HashMap;

pub fn get_balances(directives: Vec<Directive>) -> AccBal {
    let mut bals: AccBal = HashMap::new();
    for d in directives {
        match d {
            Directive::Transaction(tx) => {
                for p in tx.postings {
                    if let Some(amount) = p.amount {
                        let entry = bals.entry(p.account).or_insert_with(HashMap::new);
                        *entry.entry(amount.ccy.clone()).or_insert(0.0) += amount.number;
                    }
                }
            }
            _ => (),
        }
    }
    bals
}
