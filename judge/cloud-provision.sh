#!/bin/bash
declare API_KEY=$1
declare JUDGE_URL=$2

if [ ! $API_KEY ]; then
    echo "please provide a key."
    exit 1
fi

PROJECT_DIRECTORY="/var/judge/src"
PYTHON=$(which python3)
mkdir -p /var/judge
mkdir -p /var/log/judge

echo "installing system dependencies..."
if [ ! -f $HOME/.installdep.judge.apt ]; then
    apt-get update && apt-get install -y software-properties-common && \
    sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 5BB92C09DB82666C && \
    add-apt-repository -y ppa:fkrull/deadsnakes && \
    add-apt-repository -y ppa:openjdk-r/ppa && \
    apt-get install -y \
        build-essential \
        openjdk-7-jdk \
        pkg-config \
        python2.7 \
        python3.5 \
        python3 \
        python3-pip
    touch $HOME/.installdep.judge.apt
fi

if [ ! -d $PROJECT_DIRECTORY ]; then
    b=`realpath $(basename $0)`
    c=`dirname $b`
    d=`dirname $c`
    ln -s $c $PROJECT_DIRECTORY
else
    (cd $PROJECT_DIRECTORY; git pull origin master || true)
fi

echo "installing python dependencies..."
if [ ! -f $HOME/.installdep.judge.pip ]; then
    $PYTHON -m pip install -U pip
    $PYTHON -m pip install requests
    touch $HOME/.installdep.judge.pip
fi

# dirty hack
echo "writing systemd entry..."
PYTHON=$(which python3)
eval "echo \"$(< $PROJECT_DIRECTORY/systemd/judge.service)\"" > /etc/systemd/system/judge.service

systemctl daemon-reload
systemctl enable judge
systemctl start judge
