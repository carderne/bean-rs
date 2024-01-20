use log::debug;
use pest::iterators::Pair;

use crate::grammar::Rule;
use crate::directives::{AccBal, Badline, Directive};
use crate::book::BookError;

pub fn print_book_errors(errors: &Vec<BookError>) {
    for e in errors {
        println!("{e:?}");
    }
}

pub fn print_directives(directives: &Vec<Directive>) {
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

pub fn print_badlines(bad: Vec<Badline>) {
    for b in bad {
        eprintln!("{b}");
    }
}

pub fn print_pair(pair: &Pair<Rule>, depth: usize) {
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
            print_pair(&inner_pair, depth + 1);
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
    fn useless_print_directives() {
        let vec = Vec::new();
        print_directives(&vec)
    }
}
