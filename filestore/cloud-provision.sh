#!/bin/bash
# run this to set up the server
# only do this the first time
set -e
set -o xtrace

PROJECT_DIRECTORY="/var/filestore/src"
PYTHON=python3

echo "installing system dependencies..."
if [ ! -f $HOME/.installdep.filestore.apt ]; then
    apt-get update && apt-get install -y \
        git \
        nginx \
        python3 \
        python3-dev \
        python3-nose \
        python3-pip \
        realpath \
        systemd
    touch $HOME/.installdep.filestore.apt
fi

mkdir -p /var/filestore
mkdir -p /var/log/filestore

if [ ! -d $PROJECT_DIRECTORY ]; then
    b=`realpath $(basename $0)`
    c=`dirname $b`
    # cp -r $d $PROJECT_DIRECTORY
    ln -s $c $PROJECT_DIRECTORY
else
    (cd $PROJECT_DIRECTORY; git pull origin master || true)
fi

mkdir -p /usr/share/nginx/html/static
touch /usr/share/nginx/html/static/index.html
echo "<!-- silence is golden -->" > /usr/share/nginx/html/static/index.html
rm -rf /etc/nginx/conf.d/* /etc/nginx/sites-enabled/*
cp $PROJECT_DIRECTORY/default.conf /etc/nginx/sites-enabled/filestore

service nginx reload
service nginx restart

echo "installing python dependencies..."
if [ ! -f $HOME/.installdep.filestore.pip ]; then
    $PYTHON -m pip install -U pip
    $PYTHON -m pip install gunicorn
    $PYTHON -m pip install -r $PROJECT_DIRECTORY/requirements.txt
    touch $HOME/.installdep.filestore.pip
fi

# dirty hack
KILL=/bin/kill
eval "echo \"$(< $PROJECT_DIRECTORY/systemd/filestore.service)\"" > /etc/systemd/system/filestore.service

echo "Filestore has been deployed!"
echo "Modify the env file at /var/filestore/env."
echo "Then run"
echo
echo "systemctl --system daemon-reload && systemctl restart filestore"
echo "gucci gang"

cp env.example /var/filestore/env
systemctl --system daemon-reload && systemctl restart filestore
