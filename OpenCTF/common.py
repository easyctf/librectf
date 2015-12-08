from pymongo import MongoClient
from pymongo.errors import ConnectionFailure, InvalidName

import uuid

mongo_addr = "127.0.0.1"
mongo_port = 27017
mongo_db_name = "openctf"

__connection = None
__client = None
external_client = None

def db_conn():
    if external_client is not None:
        return external_client

    global __client, __connection
    if not __connection:
        try:
            __client = MongoClient(mongo_addr, mongo_port)
            __connection = __client[mongo_db_name]
        except ConnectionFailure:
            raise Exception("Could not connect to mongo database {} at {}:{}".format(mongo_db_name, mongo_addr, mongo_port))
        except InvalidName as error:
            raise Exception("Database {} is invalid! - {}".format(mongo_db_name, error))

    return __connection

def token():
	return str(uuid.uuid4().hex)