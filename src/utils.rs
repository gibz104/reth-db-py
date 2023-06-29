use crate::types::TableName;

use pyo3::prelude::*;
use pyo3::exceptions;
use eyre::{Result, WrapErr, ErrReport};
use std::path::Path;
use reth_db::{
    cursor::DbCursorRO,
    database::Database,
    table::Table,
    transaction::DbTx,
    mdbx::{Env, EnvKind, NoWriteMap},
    tables::{CanonicalHeaders, Headers, Transactions, TxHashNumber},
};
use reth_primitives::{
    BlockNumber,
    TxNumber,
    TxHash,
    H256
};

/// Function to create a read-only database from a given path.
pub fn read_only_db(path: &Path) -> Result<Env<NoWriteMap>, ErrReport> {
    Env::<NoWriteMap>::open(path, EnvKind::RO)
        .with_context(|| format!("Could not open database at path: {}", path.display()))
}


/// `DbTool` is a wrapper over a database (`DB`) that provides various database query functions.
pub struct DbTool<'a, DB: Database> {
    pub(crate) db: &'a DB,
}

impl<'a, DB: Database> DbTool<'a, DB> {
    /// Constructs a `DbTool` from a given database (`DB`).
    pub(crate) fn new(db: &'a DB) -> eyre::Result<Self> {
        Ok(Self { db })
    }

    /// Grabs the contents of the table within a certain index range and places the
    /// entries into a [`HashMap`][std::collections::HashMap].
    pub fn list<T: Table>(
        &self,
        skip: usize,
        len: usize,
        reverse: bool,
    ) -> Result<Vec<(T::Key, T::Value)>> {
        let data = self.db.view(|tx| {
            let mut cursor = tx.cursor_read::<T>().expect("Was not able to obtain a cursor.");

            if reverse {
                cursor.walk_back(None)?.skip(skip).take(len).collect::<Result<_, _>>()
            } else {
                cursor.walk(None)?.skip(skip).take(len).collect::<Result<_, _>>()
            }
        })?;

        data.map_err(|e| eyre::eyre!(e))
    }

    /// Grabs the content of the table for the given key
    pub fn get<T: Table>(&self, key: T::Key) -> Result<Option<T::Value>> {
        self.db.view(|tx| tx.get::<T>(key))?
            .map_err(|e| eyre::eyre!(e))
    }

    /// Returns a list of entries in a specified database table. The list consists of tuples, where each tuple
    /// represents an entry (key, value) in the database table. The list is controlled by `skip`, `len`, and `reverse` parameters.
    pub fn list_from_str(
        &self,
        table_name: TableName,
        skip: usize,
        len: usize,
        reverse: bool,
    ) -> PyResult<Vec<(String, String)>> {
        match table_name {
            TableName::CanonicalHeaders => self.list::<CanonicalHeaders>(skip, len, reverse)
                .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
                .map(|vec| vec.into_iter()
                    .map(|(key, value)| (format!("{:?}", key), format!("{:?}", value)))
                    .collect()),
            TableName::Headers => self.list::<Headers>(skip, len, reverse)
                .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
                .and_then(|vec| vec.into_iter()
                    .map(|(key, value)| {
                        serde_json::to_string(&value)
                            .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
                            .map(|v| (format!("{:?}", key), v))
                    })
                    .collect::<Result<Vec<_>, _>>()),
            TableName::Transactions => self.list::<Transactions>(skip, len, reverse)
                .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
                .and_then(|vec| vec.into_iter()
                    .map(|(key, value)| {
                        serde_json::to_string(&value)
                            .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
                            .map(|v| (format!("{:?}", key), v))
                    })
                    .collect::<Result<Vec<_>, _>>()),
            TableName::TxHashNumber => self.list::<TxHashNumber>(skip, len, reverse)
                .map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
                .map(|vec| vec.into_iter()
                    .map(|(key, value)| (format!("{:?}", key), format!("{:?}", value)))
                    .collect()),
        }
    }

    /// Returns a value from a specified database table for a given key.
    pub fn get_from_str(
        &self,
        table_name: TableName,
        key: &str,
    ) -> PyResult<String> {
        match table_name {
            TableName::CanonicalHeaders => {
                let block_number = key.parse::<u64>().map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
                let block_hash = self.get::<CanonicalHeaders>(BlockNumber::from(block_number)).map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
                serde_json::to_string(&block_hash).map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
            },
            TableName::Headers => {
                let block_number = key.parse::<u64>().map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
                let header = self.get::<Headers>(BlockNumber::from(block_number)).map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
                serde_json::to_string(&header).map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
            },
            TableName::Transactions => {
                let tx_number = key.parse::<u64>().map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
                let tx = self.get::<Transactions>(TxNumber::from(tx_number)).map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
                serde_json::to_string(&tx).map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
            },
            TableName::TxHashNumber => {
                let tx_hash = key.parse::<H256>().map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
                let tx_hash_number = self.get::<TxHashNumber>(TxHash::from(tx_hash)).map_err(|err| exceptions::PyValueError::new_err(err.to_string()))?;
                serde_json::to_string(&tx_hash_number).map_err(|err| exceptions::PyValueError::new_err(err.to_string()))
            },
        }
    }
}