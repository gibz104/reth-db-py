# <h1 align="center">reth-db-py</h1>

**Python package allowing you to interact with the Reth DB via Python. Written with Rust and Pyo3.**

**This python wrapper can access node data 15x-30x faster than local RPC calls.  Using this package, the most recent
block hash can be retrieved in ~100μs on a local reth DB.**

[![Test Suite](https://github.com/gibz104/reth-db-py/actions/workflows/CI.yml/badge.svg)](https://github.com/gibz104/reth-db-py/actions/workflows/CI.yml)
[![Py Versions](https://img.shields.io/badge/python-3.7_|_3.8_|_3.9_|_3.10-blue.svg)](https://www.python.org/downloads/)
[![Test OS](https://img.shields.io/badge/tested_on-ubuntu_|_mac_os-blue.svg)](https://github.com/gibz104/google-sheets-writer/actions/workflows/tests.yaml)

# Installation
This package has been published to PyPi and can be installed using pip:
```bash
pip install reth-db-py
````



# Assets
This package only has a single python class made available: `PyDatabaseHandler`.  

`PyDatabaseHandler` is a class used to interact with the Reth DB.  It's a wrapper around the Rust `DatabaseHandler` struct.
It has a few methods which all return json strings:
* `get_header_by_block_number`: get a single header by block number
* `get_headers_by_block_number_range`: get multiple headers by block number range
* `get_transaction_by_id`: get a single transaction by transaction id
* `get_transactions_by_id_range`: get multiple transactions by transaction id range
* `get_transactions_by_block_number_range`: get multiple transactions by block number range
* `get_block_by_number`: get a single block by block number
* `get_uncles_by_block_number`: get uncles by block number
* `get_receipts_by_transaction_id`: get receipts by transaction id
* `get_receipts_by_block_number`: get receipts by block number

```rust
impl PyDatabaseHandler {
    pub fn get_header_by_block_number(&self, number: u64) -> PyResult<String>
    pub fn get_headers_by_block_number_range(&self, start: u64, end: u64) -> PyResult<String>
    pub fn get_transaction_by_id(&self, id: u64) -> PyResult<String>
    pub fn get_transactions_by_id_range(&self, start: u64, end: u64) -> PyResult<String>
    pub fn get_transactions_by_block_number_range(&self, start: u64, end: u64) -> PyResult<String>
    pub fn get_block_by_number(&self, number: u64) -> PyResult<String>
    pub fn get_uncles_by_block_number(&self, number: u64) -> PyResult<String>
    pub fn get_receipts_by_transaction_id(&self, id: u64) -> PyResult<String>
    pub fn get_receipts_by_block_number(&self, number: u64) -> PyResult<String>
}
```

```python
class PyDatabaseHandler:
    def get_header_by_block_number(self, number: int) -> str
    def get_headers_by_block_number_range(self, start: int, end: int) -> str
    def get_transaction_by_id(self, id: int) -> str
    def get_transactions_by_id_range(self, start: int, end: int) -> str
    def get_transactions_by_block_number_range(self, start: int, end: int) -> str
    def get_block_by_number(self, number: int) -> str
    def get_uncles_by_block_number(self, number: int) -> str
    def get_receipts_by_transaction_id(self, id: int) -> str
    def get_receipts_by_block_number(self, number: int) -> str
```




# Usage
#### Import reth-db-py assets:
```python
from reth_db_py import PyDatabaseHandler
```

#### Create a PyDatabaseHandler instance:
```python
handler = PyDatabaseHandler("/path/to/db/mdbx.dat")
```

#### Get the header by block number
```python
header = handler.get_header_by_block_number(17_000_000)
```

#### Get the headers by block number range
```python
headers = handler.get_headers_by_block_number_range(17_000_000, 17_000_005)
```

#### Get transaction by id
```python
transaction = handler.get_transaction_by_id(1000)
```

#### Get transactions by id range
```python
transactions = handler.get_transactions_by_id_range(1000, 1005)
```

#### Get transactions by block number range
```python
transactions = handler.get_transactions_by_block_number_range(17_000_000, 17_000_005)
```

#### Get block by number
```python
block = handler.get_block_by_number(17_000_000)
```

#### Get uncles by block number
```python
uncles = handler.get_uncles_by_block_number(17_000_000)
```

#### Get receipts by transaction id
```python
receipts = handler.get_receipts_by_transaction_id(1000)
```

#### Get receipts by block number
```python
receipts = handler.get_receipts_by_block_number(17_000_000)
```

# Tests
Coming soon.


# Benchmarks
Speed tests between RPC calls vs. reth-db-py calls (direct db interaction) coming soon.
