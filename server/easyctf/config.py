import os
import sys
import logging
import dhall
from typing import Optional

from pathlib import Path


class Config(object):
    @classmethod
    def from_dhall_file(cls, path):
        with open(path, "r") as f:
            config = dhall.load(f)
        return Config(dhall_config=config)

    @staticmethod
    def locate_config_directory() -> Optional[Path]:
        # TODO: any other non-obvious directories we could put here?
        if (config_env := os.getenv("CONFIG_DIR")) is not None:
            path = Path(config_env)
            return path
        return None


    def __init__(self, app_root=None, testing=False, secret_key=None, dhall_config=None):
        if app_root is None:
            self.app_root = Path(
                os.path.dirname(os.path.abspath(__file__)))
        else:
            self.app_root = Path(app_root)

        self.SECRET_KEY = secret_key
        self.TESTING = testing

        if dhall_config is not None:
            # these values should have been validated (?)
            self.SECRET_KEY = dhall_config["secretKey"]
            self.ENV = dhall_config["environment"].lower()

        if env := os.getenv("ENVIRONMENT"):
            self.ENV = env
        if self.SECRET_KEY is None:
            self.SECRET_KEY = self._load_secret_key()

        self.SQLALCHEMY_DATABASE_URI = self._get_database_url()
        self.SQLALCHEMY_TRACK_MODIFICATIONS = False
        self.PREFERRED_URL_SCHEME = "https"

        self.CACHE_TYPE = "redis" # "easyctf.config.cache"
        self.CACHE_REDIS_HOST = os.getenv("CACHE_REDIS_HOST", "redis")

        self.EMAIL_VERIFICATION_REQUIRED = int(os.getenv(
            "EMAIL_VERIFICATION_REQUIRED", "1" if self.ENV == "production" else "0"))

        self.FILESTORE_SAVE_ENDPOINT = os.getenv(
            "FILESTORE_SAVE_ENDPOINT", "http://filestore:5001/save")
        self.FILESTORE_STATIC = os.getenv("FILESTORE_STATIC", "/static")

        self.DISABLE_EMAILS = os.getenv("DISABLE_EMAILS", "" if self.ENV == "production" else "1") != ""
        self.JUDGE_URL = os.getenv("JUDGE_URL", "http://127.0.0.1/")
        self.JUDGE_API_KEY = os.getenv("JUDGE_API_KEY", "")
        self.SHELL_HOST = os.getenv("SHELL_HOST", "")

        self.ADMIN_EMAIL = os.getenv("ADMIN_EMAIL", "")
        self.MAILGUN_URL = os.getenv("MAILGUN_URL", "")
        self.MAILGUN_API_KEY = os.getenv("MAILGUN_API_KEY", "")

        if self.ENV == "development":
            self.DEBUG = True
            self.TEMPLATES_AUTO_RELOAD = True

        if testing or self.ENV == "testing":
            test_db_path = os.path.join(os.path.dirname(__file__), "test.db")
            self.SQLALCHEMY_DATABASE_URI = f"sqlite:///{test_db_path}"
            if not os.path.exists(test_db_path):
                with open(test_db_path, "a"):
                    os.utime(test_db_path, None)
            self.TESTING = True
            self.WTF_CSRF_ENABLED = False

    def _load_secret_key(self):
        key = os.environ.get("SECRET_KEY")
        if key:
            return key
        if not self.SECRET_KEY:
            logging.fatal("No SECRET_KEY specified. Exiting...")
            sys.exit(1)

    @staticmethod
    def _get_database_url():
        url = os.getenv("DATABASE_URL")
        if url:
            return url

        password = os.getenv("MYSQL_ROOT_PASSWORD")
        host = os.getenv("MYSQL_HOST")
        db = os.getenv("MYSQL_DATABASE")
        return f"mysql://root:{password}@{host}:3306/{db}"
