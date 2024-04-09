use std::cmp::Ordering;

use log::debug;
use pest::iterators::Pairs;
use pest::Parser;

use crate::data::{self, DebugLine};
use crate::data::{Directive, Options};
use crate::error::{BeanError, ErrorType};
use crate::grammar::{BeanParser, Rule};
use crate::ledger::Ledger;
use crate::utils;

/// Parse the text using Pest
pub fn load(data: &str) -> Pairs<'_, Rule> {
    // The grammer has a badline option which will consume any nonsense
    // So in theory this shouldn't error!
    let mut entries = BeanParser::parse(Rule::root, data).unwrap();

    // There will always be at least an `EOI`, so this will also never error
    let entry = entries.next().unwrap();
    utils::debug_pair(&entry, 0);
    entry.into_inner()
}

/// Convert the AST Pest Pairs into a Vec of Directives
pub fn consume(entries: Pairs<'_, Rule>) -> Ledger {
    let mut errs: Vec<BeanError> = Vec::with_capacity(entries.len());
    let mut dirs: Vec<Directive> = Vec::new();
    let mut opts = Options::default();
    for entry in entries {
        debug!("{:?}\t{:?}", entry.as_rule(), entry.as_span(),);
        match entry.as_rule() {
            Rule::option => {
                opts.update_from_entry(entry);
            }
            Rule::custom => {
                dirs.push(Directive::ConfigCustom(data::ConfigCustom::from_entry(
                    entry,
                )));
            }
            Rule::query => {
                dirs.push(Directive::Query(data::Query::from_entry(entry)));
            }
            Rule::commodity => {
                dirs.push(Directive::Commodity(data::Commodity::from_entry(entry)));
            }
            Rule::open => {
                dirs.push(Directive::Open(data::Open::from_entry(entry)));
            }
            Rule::close => {
                dirs.push(Directive::Close(data::Close::from_entry(entry)));
            }
            Rule::balance => {
                dirs.push(Directive::Balance(data::Balance::from_entry(entry)));
            }
            Rule::pad => {
                dirs.push(Directive::Pad(data::Pad::from_entry(entry)));
            }
            Rule::price => {
                dirs.push(Directive::Price(data::Price::from_entry(entry)));
            }
            Rule::document => {
                dirs.push(Directive::Document(data::Document::from_entry(entry)));
            }
            Rule::note => {
                dirs.push(Directive::Note(data::Note::from_entry(entry)));
            }
            Rule::transaction => {
                dirs.push(Directive::Transaction(data::Transaction::from_entry(entry)));
            }
            Rule::EOI => {
                debug!("Hit EOI");
            }
            Rule::badline => {
                let (line, _) = entry.line_col();
                let debug = DebugLine::new(line);
                let err =
                    BeanError::new(ErrorType::Badline, &debug, "Found unparseable line", None);
                errs.push(err);
            }
            _ => {
                let (l, _) = entry.line_col();
                let debug = DebugLine::new(l);
                unreachable!("Found unexpected entry in file, abort.\n{debug}");
            }
        };
    }
    Ledger { dirs, errs, opts }
}

/// Sort the Directives by date and `order` inplace
pub fn sort(dirs: &mut [Directive]) {
    dirs.sort_by(|a, b| match a.date().cmp(b.date()) {
        Ordering::Equal => a.order().cmp(&b.order()),
        other => other,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let text = r#"2023-01-01 open Assets:Bank GBP"#;
        let entries = load(&text);
        let Ledger {
            dirs,
            errs: _,
            opts: _,
        } = consume(entries);
        let got = &dirs[0];
        match got {
            Directive::Open(_) => (),
            _ => assert!(false, "Found wrong directive type"),
        }
    }

    #[test]
    fn test_bad_consume() {
        let text = r#"
            2023-01-01 foo
        "#;
        let entries = load(&text);
        let Ledger {
            dirs: _,
            errs,
            opts: _,
        } = consume(entries);
        assert!(errs.len() == 1);
    }

    #[test]
    fn test_consume() {
        let text = r#"
            option "operating_currency" "GBP"
        "#;
        let entries = load(&text);
        consume(entries);
    }
}
