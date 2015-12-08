from flask import render_template
from OpenCTF.common import db_conn

import hashlib

def init_utils(app):
    app.jinja_env.globals.update(ctf_name=ctf_name)

def init_errors(app):
    @app.errorhandler(403)
    def forbidden(error):
        return render_template("errors/403.html"), 403
    @app.errorhandler(404)
    def page_not_found(error):
        return render_template("errors/404.html"), 404
    @app.errorhandler(405)
    def method_not_allowed(error):
        return render_template("errors/405.html"), 405
    @app.errorhandler(500)
    def general_error(error):
        return render_template("errors/500.html"), 500
    @app.errorhandler(502)
    def gateway_error(error):
        return render_template("errors/502.html"), 502
        
def get_config(key):
    db = db_conn()
    config = db.config.find_one({ "key": key })
    if config is not None:
        return config["value"]
    else:
        return None

def ctf_name():
    ctf_name = get_config("ctf_name")
    return ctf_name if ctf_name else "OpenCTF"

def is_setup():
    setup_complete = get_config("setup_complete")
    return setup_complete if setup_complete else False
    
def sha512(string):
    return hashlib.sha512(string).hexdigest()