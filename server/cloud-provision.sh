#!/bin/bash
# run this to set up the server
# only do this the first time
set -e
# set -o xtrace

REPOSITORY="git@github.com:iptq/easyctf-platform.git"
PROJECT_DIRECTORY="/var/easyctf/src"
PYTHON=python3

echo "installing system dependencies..."
if [ ! -f $HOME/.installdep.server.apt ]; then
    apt-get update && apt-get install -y \
        git \
        libffi-dev \
        libjpeg-dev \
        libmysqlclient-dev \
        libpng-dev \
        libssl-dev \
        mysql-client \
        openssh-client \
        python3 \
        python3-dev \
        python3-nose \
        python3-pip \
        realpath \
        systemd
    touch $HOME/.installdep.server.apt
fi

mkdir -p /var/easyctf
mkdir -p /var/log/easyctf

if [ ! -d $PROJECT_DIRECTORY ]; then
    # why the fuck shoul i clone if i already hav this file LMAO
    b=`realpath $(basename $0)`
    c=`dirname $b`
    d=`dirname $c`
    # cp -r $d $PROJECT_DIRECTORY
    ln -s $c $PROJECT_DIRECTORY
else
    (cd $PROJECT_DIRECTORY; git pull origin master || true)
fi

echo "installing python dependencies..."
if [ ! -f $HOME/.installdep.server.pip ]; then
    $PYTHON -m pip install -U pip
    $PYTHON -m pip install gunicorn
    $PYTHON -m pip install -r $PROJECT_DIRECTORY/requirements.txt
    touch $HOME/.installdep.server.pip
fi

# dirty hack
KILL=/bin/kill
eval "echo \"$(< $PROJECT_DIRECTORY/systemd/easyctf.service)\"" > /etc/systemd/system/easyctf.service

echo "EasyCTF has been deployed!"
echo "Modify the env file at /var/easyctf/env."
echo "Then run"
echo
echo "systemctl --system daemon-reload && systemctl restart easyctf"
echo "gucci gang"

cp env.example /var/easyctf/env
systemctl --system daemon-reload && systemctl restart easyctf
