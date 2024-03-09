use std::cmp::Ordering;

use log::debug;
use pest::error::LineColLocation;
use pest::iterators::Pairs;
use pest::Parser;

use crate::data::Directive;
use crate::data::{self, DebugLine};
use crate::error::{BeanError, ErrorType};
use crate::grammar::{BeanParser, Rule};
use crate::utils;

/// Parse the text using Pest
pub fn load(data: &str) -> Result<Pairs<'_, Rule>, BeanError> {
    let mut entries = match BeanParser::parse(Rule::root, data) {
        Ok(pairs) => Ok(pairs),
        Err(error) => {
            let (line, _) = match error.line_col {
                LineColLocation::Pos(pos) => pos,
                LineColLocation::Span(pos, _) => pos,
            };
            let debug = DebugLine::new(line);
            let err = BeanError::new(ErrorType::Parse, &debug, "Parsing error", None);
            Err(err)
        }
    }?;
    match entries.next() {
        Some(entry) => {
            utils::debug_pair(&entry, 0);
            Ok(entry.into_inner())
        }
        None => {
            let debug = DebugLine::default();
            let err = BeanError::new(ErrorType::Into, &debug, "Parsing error", None);
            Err(err)
        }
    }
}

// Convert the AST Pest Pairs into a Vec of Directives
pub fn consume(entries: Pairs<'_, Rule>) -> (Vec<Directive>, Vec<BeanError>) {
    let mut errs: Vec<BeanError> = Vec::with_capacity(entries.len());
    let mut dirs: Vec<Directive> = Vec::new();
    for entry in entries {
        debug!("{:?}\t{:?}", entry.as_rule(), entry.as_span(),);
        match entry.as_rule() {
            Rule::option => {
                dirs.push(Directive::ConfigOption(data::ConfigOption::from_entry(
                    entry,
                )));
            }
            Rule::custom => {
                dirs.push(Directive::ConfigCustom(data::ConfigCustom::from_entry(
                    entry,
                )));
            }
            Rule::query => {
                // TODO do something with queries
                let (line, _) = entry.line_col();
                let debug = DebugLine::new(line);
                debug!("Ignoring query {debug}");
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
                // TODO do something with notes
                let (line, _) = entry.line_col();
                let debug = DebugLine::new(line);
                debug!("Ignoring note {debug}");
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
                let (line, _) = entry.line_col();
                let debug = DebugLine::new(line);
                unreachable!("Found unexpected entry in file, abort.\n{debug}");
            }
        };
    }
    (dirs, errs)
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
        let entries = load(&text).unwrap();
        let (dirs, _) = consume(entries);
        let got = &dirs[0];
        match got {
            Directive::Open(_) => (),
            _ => assert!(false, "Found wrong directive type"),
        }
    }

    #[test]
    fn test_bad() {
        let text = r#"
            2023-01-01 foo
        "#;
        let entries = load(&text).unwrap();
        let (_, bad) = consume(entries);
        assert!(bad.len() == 1);
    }
}
