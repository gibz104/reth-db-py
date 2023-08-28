from unittest.mock import Mock, call, patch
import pytest
import os
from reth_db_py import PyDatabaseHandler


'''
This test file contains the benchmark tests.  All tests in this
file should be benchmark tests that use the `benchmark` pytest fixture.
'''


# Benchmark to test the speed of getting a single header (JSON)
def test_benchmark_get_header_by_block_number(benchmark):
    handler = PyDatabaseHandler(os.environ["RETH_DB_PY"])
    header = benchmark(handler.get_header_by_block_number, 1_000_000)
    # TODO: Assert header is correct

