import datetime
import logging
import logging.handlers
import os
import pkgutil

NOTSET = 0
DEBUG = 10
INFO = 20
WARNING = 30
ERROR = 40
CRITICAL = 50

def initialize_logs():
	def create_logger(name):
		new_logger = logging.getLogger(name)
		new_logger.setLevel(logging.INFO)

		base = os.path.dirname(__file__).strip("api")
		log_path = os.path.join(base, "logs")
		if not os.path.exists(log_path):
			os.mkdir(log_path)

		log_handler = logging.handlers.RotatingFileHandler(os.path.join(log_path, name + ".log"))
		new_logger.addHandler(log_handler)
	for importer, modname, ispkg in pkgutil.walk_packages(path="../api"):
		create_logger(modname)

def log(logname, message, level=INFO):
	logger = logging.getLogger(logname)
	message = "[%s] %s" % (datetime.datetime.now().strftime("%m/%d/%Y %X"), message)
	logger.log(level, message)
