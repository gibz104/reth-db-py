mod types;
mod utils;

use crate::types::{DbHandler, TableName};
use pyo3::prelude::*;


/// The main module for the reth_db_py library.
///
/// This module exposes two classes, `DbHandler` and `TableName`, which provide
/// an interface to a database.
#[pymodule]
fn reth_db_py(_py: Python, m: &PyModule) -> PyResult<()> {
    /// Handles database operations.
    m.add_class::<DbHandler>()?;
    /// Represents the name of a table in the database.
    m.add_class::<TableName>()?;
    Ok(())
}