from pymongo import MongoClient
from pymongo.database import Database
from integration_tests.utils.config import CONFIG, get_environment_specific_database_config
import os


def get_mongo_connection() -> MongoClient:
    config_environment_specific = get_environment_specific_database_config()
    client = MongoClient(config_environment_specific["host"], config_environment_specific["port"])
    return client


def _mongo_handle(key: str) -> Database:
    db_name = CONFIG["database"][key]
    return get_mongo_connection()[db_name]
    


def get_mongo_test_handle() -> Database:
    return _mongo_handle("test_db_name")


def get_mongo_dev_handle() -> Database:
    return _mongo_handle("dev_db_name")


def get_mongo_prod_handle() -> Database:
    return _mongo_handle("prod_db_name")