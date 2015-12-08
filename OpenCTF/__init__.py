from flask import Flask

def config_app(config="OpenCTF.config"):
	app = Flask("OpenCTF")
	with app.app_context():
		app.config.from_object(config)
		
		from OpenCTF.views import blueprint as views
		from OpenCTF.utils import init_utils, init_errors
		init_utils(app)
		init_errors(app)
		app.register_blueprint(views)
		return app