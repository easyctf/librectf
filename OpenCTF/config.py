import os

with open(".secret_key", "a+") as secret:
	secret.seek(0)
	key = secret.read()
	if not key:
		key = os.urandom(64)
		secret.write(key)
		secret.flush()
		
SECRET_KEY = key