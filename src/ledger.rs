use pyo3::pyclass;

use crate::data::{Directive, Options};
use crate::error::BeanError;

#[pyclass]
pub struct Ledger {
    pub dirs: Vec<Directive>,
    #[pyo3(get)]
    pub errs: Vec<BeanError>,
    #[pyo3(get)]
    pub opts: Options,
}
