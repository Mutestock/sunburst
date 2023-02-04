from pymongo import MongoClient
from utils.config import CONFIG, get_environment_specific_database_config
import os


def _mongo_handle(key: str) -> MongoClient:
    db_name = CONFIG["database"][key]
    config_environment_specific = get_environment_specific_database_config()
    
    client = MongoClient(config_environment_specific["host"], config_environment_specific["port"])
    return client[db_name]
    


def get_mongo_test_handle() -> MongoClient:
    return _mongo_handle("test_db_name")


def get_mongo_dev_handle() -> MongoClient:
    return _mongo_handle("dev_db_name")


def get_mongo_prod_handle() -> MongoClient:
    return _mongo_handle("prod_db_name")