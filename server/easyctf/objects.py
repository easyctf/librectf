from random import SystemRandom

from flask_caching import Cache
from flask_migrate import Migrate
from flask_login import LoginManager
from flask_sqlalchemy import SQLAlchemy
from raven.contrib.flask import Sentry

random = SystemRandom()
cache = Cache()
login_manager = LoginManager()
db = SQLAlchemy()
sentry = Sentry()
migrate = Migrate()
