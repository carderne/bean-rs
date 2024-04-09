use log::debug;
use pest::iterators::Pair;

use crate::data::{AccBal, Directive};
use crate::error::BeanError;
use crate::grammar::Rule;

pub fn debug_directives(directives: &Vec<Directive>) {
    for d in directives {
        debug!("{d}")
    }
}

pub fn print_directives(directives: &Vec<Directive>) {
    for d in directives {
        println!("{d}")
    }
}

pub fn print_bals(bals: AccBal) {
    println!("-- Balances --");
    for (acc, ccy_bals) in bals {
        for (ccy, amount) in ccy_bals {
            println!("{acc} {amount} {ccy}");
        }
    }
}

pub fn print_errors(errs: &Vec<BeanError>) {
    if !errs.is_empty() {
        eprintln!("-- Errors -- ");
    }
    for e in errs {
        eprintln!("{e}");
    }
}

pub fn debug_pair(pair: &Pair<Rule>, depth: usize) {
    if depth == 0 {
        debug!("full parse output");
    }

    let indent = "  ".repeat(depth);
    let inner_pairs: Vec<Pair<Rule>> = pair.clone().into_inner().collect();

    if inner_pairs.is_empty() {
        // It's a leaf node
        debug!("{}{:?}: {}", indent, pair.as_rule(), pair.as_str());
    } else {
        // Not a leaf node, just print the rule
        debug!("{}{:?}:", indent, pair.as_rule());
        for inner_pair in inner_pairs {
            debug_pair(&inner_pair, depth + 1);
        }
    }
    if depth == 0 {
        debug!("END full parse output");
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    use crate::{
        data::{CcyBal, Commodity, DebugLine},
        error::ErrorType,
    };

    use super::*;

    #[test]
    fn useless_debug_directives() {
        let comm = Commodity {
            date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            ccy: "USD".to_string(),
            meta: vec![],
            debug: DebugLine { line: 0 },
        };
        let vec = vec![Directive::Commodity(comm)];
        debug_directives(&vec)
    }

    #[test]
    fn test_print_bals() {
        let mut bals: AccBal = HashMap::new();
        let mut ccybal: CcyBal = HashMap::new();
        ccybal.insert("USD".to_string(), Decimal::new(100, 1));
        bals.insert("Assets:Checking".to_string(), ccybal);
        print_bals(bals);
    }

    #[test]
    fn test_print_errors() {
        let comm = Commodity {
            date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            ccy: "USD".to_string(),
            meta: vec![],
            debug: DebugLine { line: 0 },
        };
        let err = BeanError::new(
            ErrorType::Badline,
            &DebugLine { line: 0 },
            "",
            Some(&Directive::Commodity(comm)),
        );
        let errs = vec![err];
        print_errors(&errs);
    }
}
