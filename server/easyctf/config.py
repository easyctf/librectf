import pickle
import os
import sys
import logging

import pathlib
from cachelib import RedisCache


class CTFCache(RedisCache):
    def dump_object(self, value):
        value_type = type(value)
        if value_type in (int, int):
            return str(value).encode("ascii")
        return b"!" + pickle.dumps(value, -1)


def cache(app, config, args, kwargs):
    kwargs["host"] = app.config.get("CACHE_REDIS_HOST", "localhost")
    return CTFCache(*args, **kwargs)


class Config(object):
    def __init__(self, app_root=None, testing=False):
        if app_root is None:
            self.app_root = pathlib.Path(os.path.dirname(os.path.abspath(__file__)))
        else:
            self.app_root = pathlib.Path(app_root)

        self.TESTING = False
        self.SECRET_KEY = self._load_secret_key()
        self.SQLALCHEMY_DATABASE_URI = self._get_database_url()
        self.SQLALCHEMY_TRACK_MODIFICATIONS = False
        self.PREFERRED_URL_SCHEME = "https"

        self.CACHE_TYPE = "easyctf.config.cache"
        self.CACHE_REDIS_HOST = os.getenv("CACHE_REDIS_HOST", "redis")

        self.ENVIRONMENT = os.getenv("ENVIRONMENT", "production")
        self.EMAIL_VERIFICATION_REQUIRED = 0
        # self.EMAIL_VERIFICATION_REQUIRED = int(os.getenv(
        # "EMAIL_VERIFICATION_REQUIRED", "1" if self.ENVIRONMENT == "production" else "0"))

        self.S3_RESOURCE = os.getenv("S3_RESOURCE", "")
        self.FILESTORE_SAVE_ENDPOINT = os.getenv(
            "FILESTORE_SAVE_ENDPOINT", "http://filestore:5001/save"
        )
        self.FILESTORE_STATIC = os.getenv("FILESTORE_STATIC", "/static")

        self.JUDGE_URL = os.getenv("JUDGE_URL", "http://127.0.0.1/")
        self.JUDGE_API_KEY = os.getenv("JUDGE_API_KEY", "")
        self.SHELL_HOST = os.getenv("SHELL_HOST", "")

        self.ADMIN_EMAIL = os.getenv("ADMIN_EMAIL", "")
        self.MAILGUN_URL = os.getenv("MAILGUN_URL", "")
        self.MAILGUN_API_KEY = os.getenv("MAILGUN_API_KEY", "")

        if self.ENVIRONMENT == "development":
            self.DEBUG = True
            self.TEMPLATES_AUTO_RELOAD = True

        if testing or self.ENVIRONMENT == "testing":
            test_db_path = os.path.join(os.path.dirname(__file__), "test.db")
            self.SQLALCHEMY_DATABASE_URI = "sqlite:///%s" % test_db_path
            if not os.path.exists(test_db_path):
                with open(test_db_path, "a"):
                    os.utime(test_db_path, None)
            self.TESTING = True
            self.WTF_CSRF_ENABLED = False

    def _load_secret_key(self):
        key = os.environ.get("SECRET_KEY")
        if key:
            return key
        logging.fatal("No SECRET_KEY specified. Exiting...")
        sys.exit(1)

    @staticmethod
    def _get_database_url():
        url = os.getenv("DATABASE_URL")
        if url:
            return url
        return "mysql://root:%s@db/%s" % (
            os.getenv("MYSQL_ROOT_PASSWORD"),
            os.getenv("MYSQL_DATABASE"),
        )
