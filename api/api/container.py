from flask import Blueprint, request
from decorators import api_wrapper, admins_only, login_required, WebException

import os
import socket
import tempfile
import utils
import zipfile

blueprint = Blueprint("container", __name__)

@blueprint.route("/upload", methods=["POST"])
@api_wrapper
@admins_only
@login_required
def upload_hook():
	if "file" not in request.files:
		raise WebException("No file uploaded.")
	file = request.files["file"]

	if file.filename == "":
		raise WebException("No selected file.")

	tmpfile = os.path.join(tempfile.gettempdir(), utils.generate_string())
	file.save(tmpfile)
	if not zipfile.is_zipfile(tmpfile):
		raise WebException("This file is not a ZIP archive.")
	if "Dockerfile" not in zipfile.ZipFile(tmpfile).namelist():
		raise WebException("This ZIP does not have a Dockerfile.")

	data = send_zip_to_sandbox(tmpfile)
	if data != "ok":
		raise WebException("Problem occurred during transfer.")
	os.remove(tmpfile)

	return { "success": 1, "message": "Added." }

def send_zip_to_sandbox(filename, container_name=utils.generate_string()):
	s = socket.socket()
	length = os.stat(filename).st_size
	s.connect(("sandbox", 4000))
	s.send("%s\n" % container_name)
	s.send("%s\n" % length)

	bufsize = 32
	read = 0
	with open(filename, "rb") as file:
		while read < length:
			_bytes = file.read(bufsize)
			s.send(_bytes)
			read += bufsize

	data = s.recv(1024).strip("\n")
	return data