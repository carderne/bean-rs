use pyo3::pyclass;
use std::fmt;

use crate::data::{DebugLine, Directive};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorType {
    Badline, // un-parseable line found in input
    MultipleEmptyPostings,
    UnbalancedTransaction,
    NoAccount,
    ClosedAccount,
    DuplicateOpen,
    DuplicateClose,
    BalanceAssertion,
    UnusedPad,
    InvalidCcy,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct BeanError {
    pub ty: ErrorType,
    pub debug: DebugLine,
    pub msg: String,
}

impl BeanError {
    pub fn new(ty: ErrorType, debug: &DebugLine, msg: &str, dir: Option<&Directive>) -> Self {
        let mut msg = msg.to_owned();
        let debug = debug.clone();
        if let Some(dir) = dir {
            let m = format!("\n{dir}");
            msg.push_str(&m);
        }
        Self { ty, debug, msg }
    }
}

impl fmt::Display for BeanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "line:{debug}:  {msg}",
            debug = self.debug,
            msg = self.msg,
        )
    }
}
