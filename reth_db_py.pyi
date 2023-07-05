from enum import Enum
from typing import List, Tuple


class PyDatabaseHandler:
    def __init__(self, db_path: str):
        """
        Construct a new `PyDatabaseHandler` object.

        :param str db_path: The path to the database.
        :return: A new `PyDatabaseHandler` object.
        :rtype: PyDatabaseHandler
        """
        ...

    def get_header_by_block_number(self, number: int) -> str:
        """
        Get header by block number.

        :param int number: The block number.
        :return: A JSON string representing the header of the block.
        :rtype: str
        """
        ...

    def get_headers_by_block_number_range(self, start: int, end: int) -> str:
        """
        Get headers by block number range.

        :param int start: The start block number of the range.
        :param int end: The end block number of the range.
        :return: A JSON string representing headers of blocks within the range.
        :rtype: str
        """
        ...

    def get_transaction_by_id(self, id: int) -> str:
        """
        Get transaction by ID.

        :param int id: The transaction ID.
        :return: A JSON string representing the transaction.
        :rtype: str
        """
        ...

    def get_transactions_by_id_range(self, start: int, end: int) -> str:
        """
        Get transactions by ID range.

        :param int start: The start transaction ID of the range.
        :param int end: The end transaction ID of the range.
        :return: A JSON string representing transactions within the ID range.
        :rtype: str
        """
        ...

    def get_transactions_by_block_number_range(self, start: int, end: int) -> str:
        """
        Get transactions by block number range.

        :param int start: The start block number of the range.
        :param int end: The end block number of the range.
        :return: A JSON string representing transactions within the block number range.
        :rtype: str
        """
        ...

    def get_block_by_number(self, number: int) -> str:
        """
        Get block by number.

        :param int number: The block number.
        :return: A JSON string representing the block.
        :rtype: str
        """
        ...

    def get_uncles_by_block_number(self, number: int) -> str:
        """
        Get uncles of a block by block number.

        :param int number: The block number.
        :return: A JSON string representing the uncles of the block.
        :rtype: str
        """
        ...

    def get_receipts_by_transaction_id(self, id: int) -> str:
        """
        Get receipts by transaction ID.

        :param int id: The transaction ID.
        :return: A JSON string representing the receipts of the transaction.
        :rtype: str
        """
        ...

    def get_receipts_by_block_number(self, number: int) -> str:
        """
        Get receipts by block number.

        :param int number: The block number.
        :return: A JSON string representing the receipts of transactions within the block.
        :rtype: str
        """
        ...
