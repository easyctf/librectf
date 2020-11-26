#!/bin/bash
set -eux
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

echo "starting app $FLASK_APP..."
COMMAND=${1:-runserver}

flask db upgrade

if [ "$COMMAND" == "runserver" ]; then
    flask run --host 0.0.0.0 --port 80
    # if [ "$ENVIRONMENT" == "development" ]; then
    #     $PYTHON manage.py runserver
    # else
    #     exec gunicorn --bind="$BIND_ADDR:$BIND_PORT" -w $WORKERS 'easyctf:create_app()'
    # fi
fi
