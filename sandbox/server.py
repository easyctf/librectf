#!/usr/bin/python

from thread import start_new_thread
import io
import os
import shutil
import socket
import sys

BUILDDIR = os.path.join(os.getcwd(), "BUILDS")
if not os.path.exists(BUILDDIR):
	os.mkdir(BUILDDIR)

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

try:
	s.bind(("0.0.0.0", 4000))
except socket.error as msg:
	print "Bind failed. Error Code : " + str(msg[0]) + " Message " + msg[1]
	sys.exit()

s.listen(10)

def handler(conn):
	info = {}
	info["id"] = ""
	char = ""
	while True:
		char = conn.recv(1)
		if char == "\n": break
		info["id"] += char
	print "ID:", info["id"]

	WORKDIR = os.path.join(BUILDDIR, info["id"])
	if os.path.exists(WORKDIR):
		shutil.rmtree(WORKDIR)
	os.mkdir(WORKDIR)

	length = ""
	char = ""
	while True:
		char = conn.recv(1)
		if char == "\n": break
		length += char
	length = long(length)
	print "LENGTH:", length
	with open(os.path.join(WORKDIR, "files.zip"), "wb") as file:
		read = 0
		bufsize = 32
		while read < length:
			_bytes = conn.recv(bufsize)
			file.write(_bytes)
			read += bufsize
		file.close()

	conn.send("ok\n")
	conn.close()

while 1:
	conn, addr = s.accept()
	start_new_thread(handler, (conn,))

s.close()
