/// Only those types in the enum Directives are direct members of the Ledger.
/// The rest are children of other elements.
use std::collections::HashMap;
use std::fmt;

use chrono::NaiveDate;
use pest::iterators::{Pair, Pairs};
use rust_decimal::Decimal;

use crate::grammar::Rule;

const BASE_DATE: &str = "0001-01-01";
const DATE_FMT: &str = "%Y-%m-%d";

type Ccy = String;
pub type Account = String;

pub type CcyBal = HashMap<Ccy, Decimal>;
pub type AccBal = HashMap<Account, CcyBal>;
pub type AccStatuses = HashMap<Account, (bool, Vec<Ccy>)>;

#[derive(Clone, Debug, Default)]
pub struct DebugLine {
    pub line: usize,
}

impl DebugLine {
    pub fn new(line: usize) -> Self {
        Self { line }
    }
}

impl PartialEq for DebugLine {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl fmt::Display for DebugLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line:{line}", line = self.line)
    }
}

#[derive(Clone, Debug)]
pub struct Amount {
    pub number: Decimal,
    pub ccy: Ccy,
}

impl PartialEq for Amount {
    fn eq(&self, other: &Self) -> bool {
        // TODO get precision from context
        self.ccy == other.ccy && (self.number - other.number).abs() > Decimal::new(1, 3)
    }
}
impl Eq for Amount {}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{number} {ccy}", number = self.number, ccy = self.ccy,)
    }
}

impl Amount {
    pub fn new(number: Decimal, ccy: Ccy) -> Self {
        Self { number, ccy }
    }
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let mut number: String = pairs.next().unwrap().as_str().to_string();
        if number.contains(',') {
            number = number.replace(',', "");
        }
        let number: Decimal = match number.parse() {
            Ok(num) => num,
            Err(_) => {
                let (line, _) = entry.line_col();
                panic!("Un-parseable decimal at line:{line}");
            }
        };
        let ccy = pairs.next().unwrap().as_str().to_string();
        Self { number, ccy }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigCustom {
    date: NaiveDate,
    debug: DebugLine,
}

impl ConfigCustom {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
        let date = NaiveDate::parse_from_str(BASE_DATE, DATE_FMT).unwrap();
        Self { date, debug }
    }
}

impl fmt::Display for ConfigCustom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-- ignore custom")
    }
}

#[derive(Debug, PartialEq)]
pub struct ConfigOption {
    date: NaiveDate,
    key: String,
    val: String,
    debug: DebugLine,
}

impl ConfigOption {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let key = pairs.next().unwrap().as_str().to_string();
        let val = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
        let date = NaiveDate::parse_from_str(BASE_DATE, DATE_FMT).unwrap();
        Self {
            date,
            key,
            val,
            debug,
        }
    }
}

impl fmt::Display for ConfigOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{key} {val}", key = self.key, val = self.val,)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    key: String,
    val: String,
    debug: DebugLine,
    // TODO: parse metadata in all directives
}

impl Metadata {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let key = pairs.next().unwrap().as_str().to_string();
        let val = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
        Self { key, val, debug }
    }
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "  {key}:{val}", key = self.key, val = self.val,)
    }
}

#[derive(Debug, PartialEq)]
pub struct Commodity {
    date: NaiveDate,
    ccy: String,
    meta: Vec<Metadata>,
    debug: DebugLine,
}

impl Commodity {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str();
        let date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
        let ccy = pairs.next().unwrap().as_str().to_string();
        let mut meta: Vec<Metadata> = Vec::new();
        for pair in pairs {
            if pair.as_rule() == Rule::metadata {
                let p = Metadata::from_entry(pair);
                meta.push(p)
            }
        }
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
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
        let mut meta_string: String = String::new();
        let m_slice = &self.meta[..];
        for m in m_slice {
            let line: &str = &format!("\n{m}");
            meta_string.push_str(line);
        }
        write!(
            f,
            "{date} commodity {ccy}{meta}",
            date = self.date,
            ccy = self.ccy,
            meta = meta_string,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Open {
    date: NaiveDate,
    pub account: Account,
    pub ccys: Vec<Ccy>,
    meta: Vec<Metadata>,
    pub debug: DebugLine,
}

impl Open {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str();
        let date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
        let account = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };

        let mut ccys: Vec<Ccy> = Vec::new();
        let mut meta: Vec<Metadata> = Vec::new();

        for pair in pairs {
            match pair.as_rule() {
                Rule::ccy => {
                    let c = pair.as_str().to_owned();
                    ccys.push(c);
                }
                Rule::metadata => {
                    let m = Metadata::from_entry(pair);
                    meta.push(m);
                }
                _ => (),
            }
        }

        Self {
            date,
            account,
            ccys,
            meta,
            debug,
        }
    }
}

impl fmt::Display for Open {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{date} {account}",
            date = self.date,
            account = self.account,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Close {
    date: NaiveDate,
    pub account: Account,
    pub debug: DebugLine,
}

impl Close {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str();
        let date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
        let account = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
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
            "{date} {account}",
            date = self.date,
            account = self.account,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Balance {
    date: NaiveDate,
    pub account: Account,
    pub amount: Amount,
    pub debug: DebugLine,
}

impl Balance {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str();
        let date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
        let account = pairs.next().unwrap().as_str().to_string();
        let amount_entry = pairs.next().unwrap();
        let amount = Amount::from_entry(amount_entry);
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
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
            "{date} {account} {amount}",
            date = self.date,
            account = self.account,
            amount = self.amount,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pad {
    date: NaiveDate,
    pub account_to: Account,
    account_from: Account,
    pub debug: DebugLine,
}

impl Pad {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str();
        let date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
        let account_to = pairs.next().unwrap().as_str().to_string();
        let account_from = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
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
            "{date} {account_to} {account_from}",
            date = self.date,
            account_to = self.account_to,
            account_from = self.account_from,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Price {
    date: NaiveDate,
    commodity: String,
    amount: Amount,
    debug: DebugLine,
}

impl Price {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str();
        let date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
        let commodity = pairs.next().unwrap().as_str().to_string();
        let amount_entry = pairs.next().unwrap();
        let amount = Amount::from_entry(amount_entry);
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
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
            "{date} {commodity} {amount}",
            date = self.date,
            commodity = self.commodity,
            amount = self.amount,
        )
    }
}

#[derive(Debug, PartialEq)]
pub struct Document {
    date: NaiveDate,
    account: Account,
    path: String,
    debug: DebugLine,
}

impl Document {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str();
        let date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
        let account = pairs.next().unwrap().as_str().to_string();
        let path = pairs.next().unwrap().as_str().to_string();
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
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
            "{date} documenet {account} {path}",
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
    debug: Option<DebugLine>,
}

impl Posting {
    pub fn new(account: Account, number: Decimal, ccy: Ccy) -> Self {
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
        let amount = if pairs.peek().is_some() {
            Some(Amount::from_entry(pairs.next().unwrap()))
        } else {
            None
        };
        let (line, _) = entry.line_col();
        let debug = Some(DebugLine { line });
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
            None => String::new(),
        };

        write!(
            f,
            "  {account} {amount}",
            account = self.account,
            amount = amount_str,
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    date: NaiveDate,
    ty: String,
    payee: Option<String>,
    narration: String,
    tag: Option<String>,  // TODO can have multiple
    link: Option<String>, // TODO can have multiple
    pub postings: Vec<Posting>,
    meta: Vec<Metadata>,
    pub debug: DebugLine,
    // TODO add at_cost, at_price
}

fn get_payee_narration(pairs: &mut Pairs<Rule>) -> (Option<String>, String) {
    let first_val = pairs.next().unwrap().as_str().to_string();
    if let Some(pair) = pairs.peek() {
        if pair.as_rule() == Rule::narration {
            let narration = pairs.next().unwrap().as_str().to_string();
            return (Some(first_val), narration);
        }
    }
    (None, first_val)
}

impl Transaction {
    pub fn from_entry(entry: Pair<Rule>) -> Self {
        let mut pairs = entry.clone().into_inner();
        let date = pairs.next().unwrap().as_str();
        let date = NaiveDate::parse_from_str(date, DATE_FMT).unwrap();
        let ty = pairs.next().unwrap().as_str().to_string();
        let (payee, narration) = get_payee_narration(&mut pairs);
        let mut postings: Vec<Posting> = Vec::new();
        let mut meta: Vec<Metadata> = Vec::new();
        let mut link: Option<String> = None;
        let mut tag: Option<String> = None;
        for pair in pairs {
            match pair.as_rule() {
                Rule::posting => {
                    postings.push(Posting::from_entry(pair));
                }
                Rule::metadata => {
                    meta.push(Metadata::from_entry(pair));
                }
                Rule::link => {
                    link = Some(entry.as_str().to_owned());
                }
                Rule::tag => {
                    tag = Some(entry.as_str().to_owned());
                }
                _ => {
                    let (line, _) = entry.line_col();
                    let debug = DebugLine::new(line);
                    unreachable!("Unexpected entry in Transaction, abort.\n{debug}");
                }
            }
        }
        let (line, _) = entry.line_col();
        let debug = DebugLine { line };
        Self {
            date,
            ty,
            payee,
            narration,
            tag,
            link,
            postings,
            meta,
            debug,
        }
    }
    pub fn from_pad(pad: Pad, amount: Amount) -> Self {
        let date = pad.date;
        let ty = String::from("pad");
        let payee = None;
        let narration = String::new();
        let debug: DebugLine = DebugLine::default();
        let link = None;
        let tag = None;
        let amount2 = Some(Amount {
            number: -amount.clone().number,
            ccy: amount.clone().ccy,
        });
        let amount = Some(amount);
        let p1 = Posting {
            account: pad.account_to,
            amount: amount.clone(),
            debug: Some(debug.clone()),
        };
        let p2 = Posting {
            account: pad.account_from,
            amount: amount2,
            debug: Some(debug.clone()),
        };
        let postings = vec![p1, p2];
        let meta: Vec<Metadata> = Vec::new();
        Self {
            date,
            ty,
            payee,
            narration,
            link,
            tag,
            postings,
            meta,
            debug: debug.clone(),
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let payee_str = match &self.payee {
            Some(payee) => payee.as_str(),
            None => "",
        };

        let mut posting_string = String::new();
        let slice = &self.postings[..];
        for p in slice {
            let line: &str = &format!("\n{p}");
            posting_string.push_str(line);
        }

        let mut meta_string = String::new();
        let m_slice = &self.meta[..];
        for m in m_slice {
            let line: &str = &format!("\n{m}");
            meta_string.push_str(line);
        }

        write!(
            f,
            "{date} {ty} {payee} {narration}{meta}{postings}",
            date = self.date,
            ty = self.ty,
            payee = payee_str,
            narration = self.narration,
            meta = meta_string,
            postings = posting_string,
        )
    }
}

/// The "ledger" is made up of Directives
/// Most operations will be done by looping through a Vec of these
#[derive(Debug)]
pub enum Directive {
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
    pub fn date(&self) -> &NaiveDate {
        match self {
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
    /// This follows beancount's ordering logic, that always evaluates
    /// opens -> balances -> the rest -> documents -> closes
    pub fn order(&self) -> i8 {
        match self {
            Directive::Open(_) => -2,
            Directive::Balance(_) => -1,
            Directive::ConfigCustom(_) => 0,
            Directive::ConfigOption(_) => 0,
            Directive::Commodity(_) => 0,
            Directive::Pad(_) => 0,
            Directive::Price(_) => 0,
            Directive::Transaction(_) => 0,
            Directive::Document(_) => 1,
            Directive::Close(_) => 2,
        }
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
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
    use crate::loader;
    #[test]
    fn test_open() {
        let text = r#"2023-01-01 open Assets:Bank GBP"#;
        let entries = loader::load(&text).unwrap();
        let (dirs, _) = loader::consume(entries);
        let date = NaiveDate::parse_from_str("2023-01-01", DATE_FMT).unwrap();
        let a = &Open {
            date,
            account: String::from("Assets:Bank"),
            ccys: vec!["GBP".to_owned()],
            meta: Vec::new(),
            debug: DebugLine { line: 2 },
        };
        let got = &dirs[0];
        match got {
            Directive::Open(i) => {
                assert!(i == a);
            }
            _ => assert!(false, "Found wrong directive type"),
        }
    }
}
