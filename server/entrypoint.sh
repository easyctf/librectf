#!/bin/bash
set -e
cd /var/easyctf/src
PYTHON=python3

echo "determining bind location..."
BIND_PORT=8000
BIND_ADDR_=$(curl -w "\n" http://169.254.169.254/metadata/v1/interfaces/private/0/ipv4/address --connect-timeout 2 || printf "0.0.0.0")
BIND_ADDR=$(echo $BIND_ADDR_ | xargs)

echo "starting EasyCTF..."
COMMAND=${1:-runserver}
ENVIRONMENT=${ENVIRONMENT:-production}
WORKERS=${WORKERS:-4}
$PYTHON manage.py db upgrade
if [ "$COMMAND" == "runserver" ]; then
    if [ "$ENVIRONMENT" == "development" ]; then
        $PYTHON manage.py runserver
    else
        exec gunicorn --bind="$BIND_ADDR:$BIND_PORT" -w $WORKERS 'easyctf:create_app()'
    fi
fi
