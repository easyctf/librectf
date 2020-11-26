#!/bin/sh
set -e
PYTHON=python3

# wait for mysql to be ready
LIMIT=30
i=0
until [ $i -ge $LIMIT ]
do
  nc -z db 3306 && break
 
  i=$(( i + 1 ))
 
  echo "$i: Waiting for DB 1 second ..."
  sleep 1
done
 
if [ $i -eq $LIMIT ]
then
  echo "DB connection refused, terminating ..."
  exit 1
fi

# echo "determining bind location..."
# BIND_PORT=8000
# BIND_ADDR_=$(curl -w "\n" http://169.254.169.254/metadata/v1/interfaces/private/0/ipv4/address --connect-timeout 2 || printf "0.0.0.0")
# BIND_ADDR=$(echo $BIND_ADDR_ | xargs)

echo "starting EasyCTF..."
COMMAND=${1:-runserver}
ENVIRONMENT=${ENVIRONMENT:-production}
WORKERS=${WORKERS:-4}
flask db upgrade

if [ "$COMMAND" == "runserver" ]; then
    flask run --host 0.0.0.0 --port 8000
    # if [ "$ENVIRONMENT" == "development" ]; then
    #     $PYTHON manage.py runserver
    # else
    #     exec gunicorn --bind="$BIND_ADDR:$BIND_PORT" -w $WORKERS 'easyctf:create_app()'
    # fi
fi
