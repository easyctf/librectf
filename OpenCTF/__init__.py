from flask import Flask

def config_app():
	app = Flask("OpenCTF")
	with app.app_context():
		return app