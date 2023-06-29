use crate::utils::{read_only_db, DbTool};

use pyo3::prelude::*;
use pyo3::exceptions;
use std::path::Path;


/// Table name used for database operations.
///
/// The variants include `CanonicalHeaders`, `Headers`, `Transactions`, and `TxHashNumber`.
#[pyclass]
#[derive(Clone, Debug)]
pub enum TableName {
    CanonicalHeaders,
    Headers,
    Transactions,
    TxHashNumber,
}


/// Database Handler for performing operations on a database.
///
/// Takes a database path when instantiated.
#[pyclass]
pub struct DbHandler {
    db_path: String,
}

#[pymethods]
impl DbHandler {
    #[new]
    /// Construct a new `DbHandler` object.
    ///
    /// :param str db_path: The path to the database.
    /// :return: A new `DbHandler` object.
    /// :rtype: DbHandler
    pub fn new(db_path: String) -> Self {
        Self {
            db_path,
        }
    }

    /// List entries from a database table.
    ///
    /// :param TableName table_name: The name of the table.
    /// :param int skip: The number of entries to skip.
    /// :param int len: The number of entries to retrieve.
    /// :param bool reverse: Whether to retrieve entries in reverse order.
    /// :return: A list of tuples where each tuple represents an entry in the database table.
    /// :rtype: list
    pub fn list(&self, table_name: TableName, skip: usize, len: usize, reverse: bool) -> PyResult<Vec<(String, String)>> {
        let db_path = Path::new(self.db_path.as_str());
        let db = read_only_db(db_path).map_err(|e| exceptions::PyIOError::new_err(e.to_string()))?;
        let tool = DbTool::new(&db).map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;
        tool.list_from_str(table_name, skip, len, reverse)
    }

    /// Fetch a value from a specific table in the database by key.
    ///
    /// :param TableName table_name: The name of the table.
    /// :param str key: The key to look up in the table.
    /// :return: A string representing the value associated with the key if found.
    /// :rtype: str
    pub fn get(&self, table_name: TableName, key: String) -> PyResult<String> {
        let db_path = Path::new(self.db_path.as_str());
        let db = read_only_db(db_path).map_err(|e| exceptions::PyIOError::new_err(e.to_string()))?;
        let tool = DbTool::new(&db).map_err(|e| exceptions::PyValueError::new_err(e.to_string()))?;
        tool.get_from_str(table_name, &key)
    }
}
