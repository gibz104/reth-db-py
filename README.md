# <h1 align="center">reth-db-py</h1>

**Bare-bones Python package allowing you to interact with the Reth DB via Python. Written with Rust and Pyo3.**

**This python wrapper can access node data 2x-4x faster than local RPC calls.  Using this package, the most recent
block hash can be retrieved in ~300μs on a local reth DB.**

[![Test Suite](https://github.com/gibz104/reth-db-py/actions/workflows/CI.yml/badge.svg)](https://github.com/gibz104/reth-db-py/actions/workflows/CI.yml)
[![Py Versions](https://img.shields.io/badge/python-3.7_|_3.8_|_3.9_|_3.10-blue.svg)](https://www.python.org/downloads/)
[![Test OS](https://img.shields.io/badge/tested_on-ubuntu_|_mac_os-blue.svg)](https://github.com/gibz104/google-sheets-writer/actions/workflows/tests.yaml)

# Installation
This package has been published to PyPi and can be installed using pip:
```bash
pip install reth-db-py
````



# Assets
This package only has two assets made available: `DbHandler` and `TableName`.  

`TableName` is an enum that contains the names of the supported tables in the Reth DB.  This is used to ensure that the table name you are trying to access is valid.

```rust
pub enum TableName {
    CanonicalHeaders,
    Headers,
    Transactions,
    TxHashNumber,
}
```

`DbHandler` is a struct/class used to interact with the Reth DB.  It has two methods, `get` and `list`:
* `get` - takes a `TableName` and a `key` and returns the value associated with that key in the table
* `list` - takes a `TableName`, skip, length, and a `reverse` boolean and returns a list of keys and values from the table

```rust
impl DbHandler {
    pub fn new(db_path: String) -> Self {}
    pub fn list(&self, table_name: TableName, skip: usize, len: usize, reverse: bool) -> PyResult<Vec<(String, String)>> {}
    pub fn get(&self, table_name: TableName, key: String) -> PyResult<String> {}
}
```

```python
class DbHandler:
    def __init__(self, db_path: str)
    def list(self, table_name: TableName, skip: int, len: int, reverse: bool)
    def get(self, table_name: TableName, key: str)
```




# Usage
#### Import reth-db-py assets:
```python
from reth_db_py import DbHandler, TableName
```

#### Create a DbHandler instance:
```python
handler = DbHandler("/path/to/db/mdbx.dat")
```

#### Get a single block hash from the `CanonicalHeaders` table:
```python
header_hash = handler.get(TableName.CanonicalHeaders, '17000000')
```

#### Get 5 most recent block hashes from the `CanonicalHeaders` table:
```python
header_list = handler.list(TableName.CanonicalHeaders, 0, 5, True)
```




# Table Docs
Goal is to support all tables in Reth (currently only support 4 tables related to headers and transactions).  Reth is 
still in alpha and the database tables are subject to change. Docs on the tables can be found in the reth repo 
[here](https://github.com/paradigmxyz/reth/blob/main/docs/design/database.md).

* `CanonicalHeaders` - contains the block hash for each block number
* `Headers` - contains the header for each block hash
* `Transactions` - contains the transaction for each transaction hash
* `TxHashNumber` - contains the block number for each transaction hash




# Benchmarks
Speed tests between RPC calls vs. reth-db-py calls (direct db interaction) coming soon.
