use std::cmp::Ordering;

use pest::iterators::Pairs;
use pest::Parser;

use crate::directives;
use crate::directives::Directive;
use crate::grammar::{BeanParser, Rule};

pub fn parse(data: &str) -> Pairs<'_, Rule> {
    BeanParser::parse(Rule::root, data)
        .expect("parse failed")
        .next()
        .unwrap()
        .into_inner() // go inside the root element
}

pub fn consume(entries: Pairs<'_, Rule>) -> Vec<Directive> {
    entries
        .map(|entry| {
            eprintln!("debug:\t{:?}\t{:?}", entry.as_rule(), entry.as_span(),);
            match entry.as_rule() {
                Rule::option => {
                    Directive::ConfigOption(directives::ConfigOption::from_entry(entry))
                }
                Rule::custom => {
                    Directive::ConfigCustom(directives::ConfigCustom::from_entry(entry))
                }
                Rule::commodity => Directive::Commodity(directives::Commodity::from_entry(entry)),
                Rule::open => Directive::Open(directives::Open::from_entry(entry)),
                Rule::close => Directive::Close(directives::Close::from_entry(entry)),
                Rule::balance => Directive::Balance(directives::Balance::from_entry(entry)),
                Rule::pad => Directive::Pad(directives::Pad::from_entry(entry)),
                Rule::price => Directive::Price(directives::Price::from_entry(entry)),
                Rule::document => Directive::Document(directives::Document::from_entry(entry)),
                Rule::transaction => {
                    Directive::Transaction(directives::Transaction::from_entry(entry))
                }
                Rule::EOI => Directive::EOI(directives::EOI::from_entry(entry)),
                _ => unreachable!("no rule for this entry!"),
            }
        })
        .collect()
}

pub fn sort(directives: &mut Vec<Directive>) {
    directives.sort_by(|a, b| match a.date().cmp(&b.date()) {
        Ordering::Equal => a.order().cmp(&b.order()),
        other => other,
    });
}
