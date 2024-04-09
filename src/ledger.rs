use crate::data::{Directive, Options};
use crate::error::BeanError;

pub struct Ledger {
    pub dirs: Vec<Directive>,
    pub errs: Vec<BeanError>,
    pub opts: Options,
}
