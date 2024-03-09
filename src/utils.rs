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
    use super::*;
    #[test]
    fn useless_debug_directives() {
        let vec = Vec::new();
        debug_directives(&vec)
    }
}
