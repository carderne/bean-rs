use std::collections::HashMap;
use std::fmt;

use pest::iterators::{Pair, Pairs};

use crate::grammar::Rule;

const BASE_DATE: &str = "0001-01-01";

type Ccy = String;
type Account = String;

pub type CcyBal = HashMap<String, f64>;
pub type AccBal = HashMap<String, CcyBal>;

#[derive(Clone, Debug, Default)]
pub struct Debug {
    line: usize,
}

impl PartialEq for Debug {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Display for Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L{line:0>4} ", line = self.line)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Amount {
    pub number: f64,
    pub ccy: Ccy,
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{number} {ccy}", number = self.number, ccy = self.ccy,)
    }
}

impl Amount {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let number: f64 = pairs.next().unwrap().as_str().parse().unwrap();
        let ccy = pairs.next().unwrap().as_str().to_string();
        Self { number, ccy }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigCustom {
    date: String,
    debug: Debug,
}

impl ConfigCustom {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date: String::from(BASE_DATE),
            debug,
        }
    }
}

impl fmt::Display for ConfigCustom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{debug}-- ignore custom", debug = self.debug)
    }
}

#[derive(Debug, PartialEq)]
pub struct EOI {
    date: String,
    debug: Debug,
}

impl EOI {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date: String::from(BASE_DATE),
            debug,
        }
    }
}

impl fmt::Display for EOI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{debug}-- EOI", debug = self.debug)
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigOption {
    date: String,
    key: String,
    val: String,
    debug: Debug,
}

impl ConfigOption {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let key = pairs.next().unwrap().as_str().to_string();
        let val = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date: String::from(BASE_DATE),
            key,
            val,
            debug,
        }
    }
}

impl fmt::Display for ConfigOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{debug}{key} {val}",
            debug = self.debug,
            key = self.key,
            val = self.val,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Metadata {
    key: String,
    val: String,
    debug: Debug,
}

impl Metadata {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let key = pairs.next().unwrap().as_str().to_string();
        let val = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self { key, val, debug }
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{debug}  {key}:{val}",
            debug = self.debug,
            key = self.key,
            val = self.val,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Commodity {
    date: String,
    ccy: String,
    meta: Vec<Metadata>,
    debug: Debug,
}

impl Commodity {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str().to_string();
        let ccy = pairs.next().unwrap().as_str().to_string();
        let mut meta: Vec<Metadata> = Vec::new();
        while let Some(pair) = pairs.next() {
            if pair.as_rule() == Rule::metadata {
                let p = Metadata::from_entry(pair);
                meta.push(p)
            }
        }
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date,
            ccy,
            meta,
            debug,
        }
    }
}

impl fmt::Display for Commodity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut meta_string: String = "".to_owned();
        let m_slice = &self.meta[..];
        for m in m_slice {
            let line: &str = &format!("\n{m}");
            meta_string.push_str(line);
        }
        write!(
            f,
            "{debug}{date} commodity {ccy}{meta}",
            debug = self.debug,
            date = self.date,
            ccy = self.ccy,
            meta = meta_string,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Open {
    date: String,
    account: Account,
    debug: Debug,
}

impl Open {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str().to_string();
        let account = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date,
            account,
            debug,
        }
    }
}

impl fmt::Display for Open {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{debug}{date} {account}",
            debug = self.debug,
            date = self.date,
            account = self.account,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Close {
    date: String,
    account: Account,
    debug: Debug,
}

impl Close {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str().to_string();
        let account = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date,
            account,
            debug,
        }
    }
}

impl fmt::Display for Close {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{debug}{date} {account}",
            debug = self.debug,
            date = self.date,
            account = self.account,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Balance {
    date: String,
    account: Account,
    amount: Amount,
    debug: Debug,
}

impl Balance {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str().to_string();
        let account = pairs.next().unwrap().as_str().to_string();
        let amount_entry = pairs.next().unwrap();
        let amount = Amount::from_entry(amount_entry);
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date,
            account,
            amount,
            debug,
        }
    }
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{debug}{date} {account} {amount}",
            debug = self.debug,
            date = self.date,
            account = self.account,
            amount = self.amount,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Pad {
    date: String,
    account_to: Account,
    account_from: Account,
    debug: Debug,
}

impl Pad {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str().to_string();
        let account_to = pairs.next().unwrap().as_str().to_string();
        let account_from = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date,
            account_to,
            account_from,
            debug,
        }
    }
}

impl fmt::Display for Pad {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{debug}{date} {account_to} {account_from}",
            debug = self.debug,
            date = self.date,
            account_to = self.account_to,
            account_from = self.account_from,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Price {
    date: String,
    commodity: String,
    amount: Amount,
    debug: Debug,
}

impl Price {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str().to_string();
        let commodity = pairs.next().unwrap().as_str().to_string();
        let amount_entry = pairs.next().unwrap();
        let amount = Amount::from_entry(amount_entry);
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date,
            commodity,
            amount,
            debug,
        }
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{debug}{date} {commodity} {amount}",
            debug = self.debug,
            date = self.date,
            commodity = self.commodity,
            amount = self.amount,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Document {
    date: String,
    account: Account,
    path: String,
    debug: Debug,
}

impl Document {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str().to_string();
        let account = pairs.next().unwrap().as_str().to_string();
        let path = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date,
            account,
            path,
            debug,
        }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{debug}{date} documenet {account} {path}",
            debug = self.debug,
            date = self.date,
            account = self.account,
            path = self.path,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Posting {
    pub account: Account,
    pub amount: Option<Amount>,
    debug: Option<Debug>,
}

impl Posting {
    pub fn new(account: Account, number: f64, ccy: Ccy) -> Self {
        let amount = Some(Amount { number, ccy });
        let debug = Default::default();
        Self {
            account,
            amount,
            debug,
        }
    }
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let account = pairs.next().unwrap().as_str().to_string();
        let amount = if let Some(_) = pairs.peek() {
            Some(Amount::from_entry(pairs.next().unwrap()))
        } else {
            None
        };
        let (line, _) = entry.line_col();
        let debug = Some(Debug { line });
        Self {
            account,
            amount,
            debug,
        }
    }
}

impl fmt::Display for Posting {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let amount_str = match &self.amount {
            Some(amount) => amount.to_string(),
            None => String::from(""),
        };

        let debug: Debug = Debug::default();
        let debug = self.debug.as_ref().unwrap_or(&debug);

        write!(
            f,
            "{debug}  {account} {amount}",
            debug = debug,
            account = self.account,
            amount = amount_str,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    date: String,
    ty: String,
    payee: Option<String>,
    narration: String,
    pub postings: Vec<Posting>,
    meta: Vec<Metadata>,
    debug: Debug,
}

pub fn get_payee_narration(pairs: &mut Pairs<Rule>) -> (Option<String>, String) {
    let first_val = pairs.next().unwrap().as_str().to_string();
    if pairs.peek().unwrap().as_rule() == Rule::narration {
        let narration = pairs.next().unwrap().as_str().to_string();
        return (Some(first_val), narration);
    } else {
        return (None, first_val);
    }
}

impl Transaction {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str().to_string();
        let ty = pairs.next().unwrap().as_str().to_string();
        let (payee, narration) = get_payee_narration(&mut pairs);
        let mut postings: Vec<Posting> = Vec::new();
        let mut meta: Vec<Metadata> = Vec::new();
        while let Some(pair) = pairs.next() {
            if pair.as_rule() == Rule::posting {
                postings.push(Posting::from_entry(pair));
            } else if pair.as_rule() == Rule::metadata {
                meta.push(Metadata::from_entry(pair));
            }
        }
        let (line, _) = entry.line_col();
        let debug = Debug { line };
        Self {
            date,
            ty,
            payee,
            narration,
            postings,
            meta,
            debug,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let payee_str = match &self.payee {
            Some(payee) => payee.as_str(),
            None => "",
        };

        let mut posting_string: String = "".to_owned();
        let slice = &self.postings[..];
        for p in slice {
            let line: &str = &format!("\n{p}");
            posting_string.push_str(line);
        }

        let mut meta_string: String = "".to_owned();
        let m_slice = &self.meta[..];
        for m in m_slice {
            let line: &str = &format!("\n{m}");
            meta_string.push_str(line);
        }

        write!(
            f,
            "{debug}{date} {ty} {payee} {narration}{meta}{postings}",
            debug = self.debug,
            date = self.date,
            ty = self.ty,
            payee = payee_str,
            narration = self.narration,
            meta = meta_string,
            postings = posting_string,
        )
    }
}

pub enum Directive {
    EOI(EOI),
    ConfigCustom(ConfigCustom),
    ConfigOption(ConfigOption),
    Commodity(Commodity),
    Open(Open),
    Close(Close),
    Balance(Balance),
    Pad(Pad),
    Price(Price),
    Document(Document),
    Transaction(Transaction),
}

impl Directive {
    pub fn date(&self) -> &str {
        match self {
            Directive::EOI(d) => &d.date,
            Directive::ConfigCustom(d) => &d.date,
            Directive::ConfigOption(d) => &d.date,
            Directive::Commodity(d) => &d.date,
            Directive::Open(d) => &d.date,
            Directive::Close(d) => &d.date,
            Directive::Balance(d) => &d.date,
            Directive::Pad(d) => &d.date,
            Directive::Price(d) => &d.date,
            Directive::Document(d) => &d.date,
            Directive::Transaction(d) => &d.date,
        }
    }
    pub fn order(&self) -> i8 {
        match self {
            Directive::EOI(_) => 0,
            Directive::ConfigCustom(_) => 0,
            Directive::ConfigOption(_) => 0,
            Directive::Commodity(_) => 0,
            Directive::Open(_) => -2,
            Directive::Close(_) => 2,
            Directive::Balance(_) => -1,
            Directive::Pad(_) => 0,
            Directive::Price(_) => 0,
            Directive::Document(_) => 1,
            Directive::Transaction(_) => 0,
        }
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Directive::EOI(d) => write!(f, "{d}"),
            Directive::ConfigCustom(d) => write!(f, "{d}"),
            Directive::ConfigOption(d) => write!(f, "{d}"),
            Directive::Commodity(d) => write!(f, "{d}"),
            Directive::Open(d) => write!(f, "{d}"),
            Directive::Close(d) => write!(f, "{d}"),
            Directive::Balance(d) => write!(f, "{d}"),
            Directive::Pad(d) => write!(f, "{d}"),
            Directive::Price(d) => write!(f, "{d}"),
            Directive::Document(d) => write!(f, "{d}"),
            Directive::Transaction(d) => write!(f, "{d}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    #[test]
    fn test_open() {
        let text = r#"2023-01-01 open Assets:Bank GBP"#;
        let entries = parser::parse(&text);
        let dirs = parser::consume(entries);
        let a = &Open {
            date: String::from("2023-01-01"),
            account: String::from("Assets:Bank"),
            debug: Debug { line: 2 },
        };
        let got = &dirs[0];
        match got {
            Directive::Open(i) => {
                assert!(i == a);
            }
            _ => panic!("Found wrong directive type"),
        }
    }
}
