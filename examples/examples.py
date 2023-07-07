# Import the DbHandler and TableName from reth_db_py
from reth_db_py import PyDatabaseHandler
from time import perf_counter_ns
import requests
import json


# Path to the MDBX database file (created by Reth process)
handler = PyDatabaseHandler("/mnt/Node_Data/docker/volumes/local_reth_nodedata/_data/reth/db/mdbx.dat")


# Get the header by block number
header = handler.get_header_by_block_number(17_000_000)

# Get the headers by block number range
headers = handler.get_headers_by_block_number_range(17_000_000, 17_000_005)

# Get transaction by id
transaction = handler.get_transaction_by_id(1000)

# Get transactions by id range
transactions = handler.get_transactions_by_id_range(1000, 1005)

# Get transactions by block number range
transactions = handler.get_transactions_by_block_number_range(17_000_000, 17_000_005)

# Get block by number
block = handler.get_block_by_number(17_000_000)

# Get uncles by block number
uncles = handler.get_uncles_by_block_number(17_000_000)

# Get receipts by transaction id
receipts = handler.get_receipts_by_transaction_id(1000)

# Get receipts by block number
receipts = handler.get_receipts_by_block_number(17_000_000)



# HTTP RPC Call vs. Reth-db-py call
url = "http://127.0.0.1:8545"
headers = {
    'Content-Type': 'application/json',
}
payload = {
    "method": "eth_blockNumber",
    "params": [],
    "id": 1,
    "jsonrpc": "2.0"
}
start_time = perf_counter_ns()
response = requests.post(url, headers=headers, data=json.dumps(payload))
duration_rpc = (perf_counter_ns() - start_time) / 1_000_000.0
print(f'RPC time: {duration_rpc} ms')

# Reth-db-py call to get block hash
start_time = perf_counter_ns()
header_hash = handler.get_header_by_block_number(17_000_000)
duration = (perf_counter_ns() - start_time) / 1_000_000.0
print(f'reth_db_py time: {duration} ms')

print(f'reth-db-py is {duration_rpc / duration} times faster than RPC')
