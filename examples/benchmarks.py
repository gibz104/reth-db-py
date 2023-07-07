import timeit
import seaborn as sns
import pandas as pd
import matplotlib.pyplot as plt
import statistics
from web3 import Web3
import requests
import json
from reth_db_py import PyDatabaseHandler


def get_block_local_web3(block_number: int):
    # Connect to Ethereum node using Infura
    w3 = Web3(Web3.HTTPProvider("http://127.0.0.1:8545"))
    # Get block data
    _block = w3.eth.get_block(block_number, full_transactions=True)


def get_block_alchemy(block_number: int):
    url = "https://eth-mainnet.g.alchemy.com/v2/zpAe-NVcByLAtwahyJUvoSdpAHcmDEqr"
    headers = {"Content-Type": "application/json"}
    # The method eth_getBlockByNumber gets the block data by its number
    data = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_getBlockByNumber",
        "params": [hex(block_number), True]
    }
    # Get block data
    _response = requests.post(url, headers=headers, data=json.dumps(data)).json()


def get_block_reth_db_py(block_number: int):
    # Path to the MDBX database file (created by Reth process)
    handler = PyDatabaseHandler("/mnt/Node_Data/docker/volumes/local_reth_nodedata/_data/reth/db/mdbx.dat")
    # Get block data
    _block = handler.get_block_by_number(block_number)


functions = [get_block_local_web3, get_block_alchemy, get_block_reth_db_py]
num_runs = 1_000
results = []

start_block = 16_000_000
for function in functions:
    function_name = function.__name__
    run_times = [timeit.timeit(lambda: function(start_block + i), number=1) for i in range(num_runs)]
    average_run_time = sum(run_times) / len(run_times)
    std_dev_run_time = statistics.stdev(run_times)

    print(f"Function: {function_name}")
    print(f"Average run time over {num_runs} runs: {average_run_time * 1000} milliseconds")
    print(f"Standard deviation of run time: {std_dev_run_time * 1000} milliseconds")
    print("\n")

    # Modify function name to include average run time
    function_name += f' (Avg: {average_run_time * 1000:.2f} ms)'

    # Add to results
    for i, run_time in enumerate(run_times):
        results.append(
            {'Function': function_name, 'Run Time': run_time * 1000, 'Run Count': i+1})

# Create DataFrame from results
df = pd.DataFrame(results)

# Plotting
sns.lineplot(data=df, x='Run Count', y='Run Time', hue='Function')
plt.yscale('log')  # Set y-axis to logarithmic scale
plt.ylabel('Run Time (ms)')  # Update y-axis label
plt.title('Run Times Distribution')
plt.show()
