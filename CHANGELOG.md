[//]: # ("Added", "Changed", "Deprecated", "Removed", "Fixed", and "Security")

## [0.1.4] - 2023-07-05

### Added
- Python type annotations in reth_db_py.pyi
- Added class `PyDatabaseHandler`
- Added methods `get_header_by_block_number`, `get_headers_by_block_number_range`, `get_transaction_by_id`,
  `get_transactions_by_id_range`, `get_transactions_by_block_number_range`, `get_block_by_number`,
  `get_uncles_by_block_number`, `get_receipts_by_transaction_id`, and `get_receipts_by_block_number`.
- Added changelog

### Removed
- Removed `DbHandler` class
- Removed `TableName` enum

### Changed
- Switched to using reth providers to access database

## [0.1.3] - 2023-06-29

### Added
- Initial release of the project.

