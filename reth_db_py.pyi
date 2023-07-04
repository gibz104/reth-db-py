from enum import Enum
from typing import List, Tuple


class TableName(Enum):
    """Table name used for database operations."""
    CanonicalHeaders = 0
    Headers = 1
    Transactions = 2
    TxHashNumber = 3


class DbHandler:
    def __init__(self, db_path: str):
        """
        Construct a new `DbHandler` object.
       
        :param str db_path: The path to the database.
        :return: A new `DbHandler` object.
        :rtype: DbHandler
        """
        ...

    def list(self, table_name: TableName, skip: int, len: int, reverse: bool) -> List[Tuple[str, str]]:
        """
        List entries from a database table.
       
        :param TableName table_name: The name of the table.
        :param int skip: The number of entries to skip.
        :param int len: The number of entries to retrieve.
        :param bool reverse: Whether to retrieve entries in reverse order.
        :return: A list of tuples where each tuple represents an entry in the database table.
        :rtype: list
        """
        ...

    def get(self, table_name: TableName, key: str) -> str:
        """
        Fetch a value from a specific table in the database by key.
       
        :param TableName table_name: The name of the table.
        :param str key: The key to look up in the table.
        :return: A string representing the value associated with the key if found.
        :rtype: str
        """
        ...
