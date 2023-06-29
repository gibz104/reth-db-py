# Import the DbHandler and TableName from reth_db_py
from reth_db_py import DbHandler, TableName
from time import perf_counter_ns
import requests
import json


# Path to the MDBX database file (created by Reth process)
handler = DbHandler("/mnt/Node_Data/docker/volumes/local_reth_nodedata/_data/reth/db/mdbx.dat")


# Get a specific header
header_hash = handler.get(TableName.Headers, '17000000')
print(f'Header hash: {header_hash}')


# List most recent header data
header_list = handler.list(TableName.Headers, 0, 1, True)
print(f'Header list: {header_list}')


# HTTP RPC Call vs. Reth-db-py call
start_time = perf_counter_ns()
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
response = requests.post(url, headers=headers, data=json.dumps(payload))
duration = (perf_counter_ns() - start_time) / 1_000_000.0
print(f'RPC time: {duration} ms')

# Reth-db-py call to get block hash
start_time = perf_counter_ns()
header_hash = handler.list(TableName.CanonicalHeaders, 0, 1, True)
duration = (perf_counter_ns() - start_time) / 1_000_000.0
print(f'reth_db_py time: {duration} ms')


