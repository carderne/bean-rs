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
                Rule::option => Directive::ConfigOption(directives::ConfigOption::new(entry)),
                Rule::custom => Directive::ConfigCustom(directives::ConfigCustom::new(entry)),
                Rule::commodity => Directive::Commodity(directives::Commodity::new(entry)),
                Rule::open => Directive::Open(directives::Open::new(entry)),
                Rule::close => Directive::Close(directives::Close::new(entry)),
                Rule::balance => Directive::Balance(directives::Balance::new(entry)),
                Rule::pad => Directive::Pad(directives::Pad::new(entry)),
                Rule::price => Directive::Price(directives::Price::new(entry)),
                Rule::transaction => Directive::Transaction(directives::Transaction::new(entry)),
                Rule::EOI => Directive::EOI(directives::EOI::new(entry)),
                _ => unreachable!("no rule for this entry!"),
            }
        })
        .collect()
}
