mod types;
mod utils;

use crate::types::{PyDatabaseHandler};
use pyo3::prelude::*;


/// The main module for the reth_db_py library.
///
/// This module exposes two classes, `DbHandler` and `TableName`, which provide
/// an interface to a database.
#[pymodule]
fn reth_db_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyDatabaseHandler>()?;
    Ok(())
}