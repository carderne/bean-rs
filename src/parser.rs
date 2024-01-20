use std::cmp::Ordering;
use std::fmt;

use log::debug;
use pest::error::LineColLocation;
use pest::iterators::Pairs;
use pest::Parser;

use crate::directives::Directive;
use crate::directives::{self, Badline};
use crate::grammar::{BeanParser, Rule};
use crate::utils;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ParseErrorType {
    Parse, // parse error from Pest
    Into,  // error while going into `root` pair
}

/// Could possibly just use Pest's `Error`
/// but this seems a bit nicer
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ParseError {
    ty: ParseErrorType,
    line: usize,
    col: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ParseError: ({ty:?}) L{line} C{col}",
            ty = self.ty,
            line = self.line,
            col = self.col,
        )
    }
}

/// Parse the text using Pest
pub fn parse(data: &str) -> Result<Pairs<'_, Rule>, ParseError> {
    let mut entries = match BeanParser::parse(Rule::root, data) {
        Ok(pairs) => Ok(pairs),
        Err(error) => {
            let (line, col) = match error.line_col {
                LineColLocation::Pos(pos) => pos,
                LineColLocation::Span(pos, _) => pos,
            };
            let ty = ParseErrorType::Parse;
            Err(ParseError { ty, line, col })
        }
    }?;
    match entries.next() {
        Some(entry) => {
            utils::print_pair(&entry, 0);
            Ok(entry.into_inner())
        }
        None => Err(ParseError {
            ty: ParseErrorType::Into,
            line: 0,
            col: 0,
        }),
    }
}

// Convert the AST Pest Pairs into a Vec of Directives
pub fn consume(entries: Pairs<'_, Rule>) -> (Vec<Directive>, Vec<Badline>) {
    let mut bad: Vec<Badline> = Vec::with_capacity(entries.len());
    let mut dirs: Vec<Directive> = Vec::new();
    for entry in entries {
        debug!("{:?}\t{:?}", entry.as_rule(), entry.as_span(),);
        match entry.as_rule() {
            Rule::option => {
                dirs.push(Directive::ConfigOption(
                    directives::ConfigOption::from_entry(entry),
                ));
            }
            Rule::custom => {
                dirs.push(Directive::ConfigCustom(
                    directives::ConfigCustom::from_entry(entry),
                ));
            }
            Rule::commodity => {
                dirs.push(Directive::Commodity(directives::Commodity::from_entry(
                    entry,
                )));
            }
            Rule::open => {
                dirs.push(Directive::Open(directives::Open::from_entry(entry)));
            }
            Rule::close => {
                dirs.push(Directive::Close(directives::Close::from_entry(entry)));
            }
            Rule::balance => {
                dirs.push(Directive::Balance(directives::Balance::from_entry(entry)));
            }
            Rule::pad => {
                dirs.push(Directive::Pad(directives::Pad::from_entry(entry)));
            }
            Rule::price => {
                dirs.push(Directive::Price(directives::Price::from_entry(entry)));
            }
            Rule::document => {
                dirs.push(Directive::Document(directives::Document::from_entry(entry)));
            }
            Rule::transaction => {
                dirs.push(Directive::Transaction(directives::Transaction::from_entry(
                    entry,
                )));
            }
            Rule::EOI => {
                debug!("Hit EOI");
            }
            Rule::badline => {
                let (line, _) = entry.line_col();
                bad.push(directives::Badline::new(line));
            }
            _ => unreachable!("no rule for this entry!"),
        };
    }
    (dirs, bad)
}

/// Sort the Directives by date and `order` inplace
pub fn sort(directives: &mut [Directive]) {
    directives.sort_by(|a, b| match a.date().cmp(b.date()) {
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
        let entries = parse(&text).unwrap();
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
        let entries = parse(&text).unwrap();
        let (_, bad) = consume(entries);
        assert!(bad.len() == 1);
    }
}
