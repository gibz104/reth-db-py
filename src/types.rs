use crate::utils::DatabaseHandler;
use pyo3::prelude::*;
use pyo3::exceptions;


#[pyclass]
pub struct PyDatabaseHandler {
    provider: DatabaseHandler,
}

#[pymethods]
impl PyDatabaseHandler {
    #[new]
    pub fn new(db_path: &str) -> PyResult<Self> {
        match DatabaseHandler::new(db_path.to_string()) {
            Ok(provider) => Ok(Self { provider }),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub fn get_header_by_block_number(&self, number: u64) -> PyResult<String> {
        match self.provider.get_header_by_block_number(number) {
            Ok(header) => Ok(header),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub fn get_headers_by_block_number_range(&self, start: u64, end: u64) -> PyResult<String> {
        match self.provider.get_headers_by_block_number_range(start, end) {
            Ok(headers) => Ok(headers),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub(crate) fn get_transaction_by_id(&self, id: u64) -> PyResult<String> {
        match self.provider.get_transaction_by_id(id) {
            Ok(transaction) => Ok(transaction),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub(crate) fn get_transactions_by_id_range(&self, start: u64, end: u64) -> PyResult<String> {
        match self.provider.get_transactions_by_id_range(start, end) {
            Ok(transactions) => Ok(transactions),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub(crate) fn get_transactions_by_block_number_range(&self, start: u64, end: u64) -> PyResult<String> {
        match self.provider.get_transactions_by_block_number_range(start, end) {
            Ok(transactions) => Ok(transactions),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub(crate) fn get_block_by_number(&self, number: u64) -> PyResult<String> {
        match self.provider.get_block_by_number(number) {
            Ok(block) => Ok(block),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub(crate) fn get_uncles_by_block_number(&self, number: u64) -> PyResult<String> {
        match self.provider.get_uncles_by_block_number(number) {
            Ok(uncles) => Ok(uncles),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub(crate) fn get_receipts_by_transaction_id(&self, id: u64) -> PyResult<String> {
        match self.provider.get_receipts_by_transaction_id(id) {
            Ok(receipts) => Ok(receipts),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }

    pub(crate) fn get_receipts_by_block_number(&self, number: u64) -> PyResult<String> {
        match self.provider.get_receipts_by_block_number(number) {
            Ok(receipts) => Ok(receipts),
            Err(err) => Err(PyErr::new::<exceptions::PyException, _>(format!("{:?}", err))),
        }
    }
}
