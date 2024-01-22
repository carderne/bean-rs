use std::fmt;

use crate::directives::Directive;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorType {
    Parse,        // parse error from Pest
    Into,         // error while going into `root` pair
    Badline,      // un-parseable line found in input
    UnknownEntry, // `consume` match statement exhausted
    MultipleEmptyPostings,
    EmptyPosting,
    UnbalancedTransaction,
    NoAccount,
    ClosedAccount,
    BalanceAssertion,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct BeanError {
    ty: ErrorType, // not yet used anywhere
    file: String,  // TODO not yet populated anywhere!
    line: usize,
    msg: String,
    dir: Option<Directive>,
}

impl BeanError {
    pub fn new(
        ty: ErrorType,
        file: &str,
        line: usize,
        msg: &str,
        dir: Option<Directive>,
    ) -> Self {
        let file = file.to_owned();
        let msg = msg.to_owned();
        Self {
            ty,
            file,
            line,
            msg,
            dir,
        }
    }
}

impl fmt::Display for BeanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let dir = match &self.dir {
            Some(d) => format!("\n{d}"),
            None => String::new(),
        };
        write!(
            f,
            "line:{line}:  {msg}{dir}",
            line = self.line,
            msg = self.msg,
            dir = dir,
        )
    }
}
