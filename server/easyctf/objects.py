from random import SystemRandom

import boto3
from flask_caching import Cache
from flask_migrate import Migrate
from flask_login import LoginManager
from flask_sqlalchemy import SQLAlchemy
from raven.contrib.flask import Sentry

class S3Wrapper:
    def __init__(self):
        self.client = None

    def init_app(self, app):
        s3_resource = app.config.get("S3_RESOURCE")
        self.client = boto3.resource(
            's3',
            endpoint_url = s3_resource,
        )

random = SystemRandom()
cache = Cache()
login_manager = LoginManager()
db = SQLAlchemy()
sentry = Sentry()
migrate = Migrate()
s3 = S3Wrapper()
