#!/bin/bash
set -e
cd /var/filestore/src

PYTHON=/usr/bin/python3

echo "determining bind location..."
BIND_PORT=${FILESTORE_PORT:-8000}
PRIVATE_BIND_ADDR_=$(curl -w "\n" http://169.254.169.254/metadata/v1/interfaces/private/0/ipv4/address --connect-timeout 2 || printf "0.0.0.0")
PRIVATE_BIND_ADDR=$(echo $BIND_ADDR_ | xargs)
PUBLIC_BIND_ADDR_=$(curl -w "\n" http://169.254.169.254/metadata/v1/interfaces/public/0/ipv4/address --connect-timeout 2 || printf "0.0.0.0")
PUBLIC_BIND_ADDR=$(echo $BIND_ADDR_ | xargs)

WORKERS=${WORKERS:-4}
ENVIRONMENT=${ENVIRONMENT:-production}
service nginx start
if [ "$ENVIRONMENT" == "development" ]; then
	$PYTHON app.py
else
	exec gunicorn --bind="$PRIVATE_BIND_ADDR:$BIND_PORT" --bind="$PUBLIC_BIND_ADDR:$BIND_PORT" -w $WORKERS app:app
fi
