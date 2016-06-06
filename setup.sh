#!/bin/bash

MYSQL_ROOT_PASSWORD="i_hate_passwords"
MYSQL_CNF_FILE=.my.cnf

echo "Updating system..."
apt-get -y update
apt-get -y upgrade

echo "Preparing for MySQL installation..."
debconf-set-selections <<< "mysql-server mysql-server/root_password password $MYSQL_ROOT_PASSWORD"
debconf-set-selections <<< "mysql-server mysql-server/root_password_again password $MYSQL_ROOT_PASSWORD"
debconf-set-selections <<< "postfix postfix/mailname string $HOST"
debconf-set-selections <<< "postfix postfix/main_mailer_type string 'Internet Site'"

echo "Installing dependencies..."
apt-get -y install python python-nose dos2unix python-pip libjpeg-dev python-dev libmysqlclient-dev nginx mysql-server memcached tmux postfix mailutils

echo "Installing pip dependencies..."
pip install -r scripts/requirements.txt

echo "Creating environmental variables..."
echo "PATH=$PATH:$(pwd)" >> /etc/profile
echo "CTFDIR=$(pwd)" >> /etc/profile
source /etc/profile

echo "Setting up nginx..."
cp ctf.nginx /etc/nginx/sites-enabled/ctf
rm /etc/nginx/sites-*/default
sudo service nginx restart

echo "Setting up MySQL..."
mysql -u root -p"$MYSQL_ROOT_PASSWORD" -e "CREATE DATABASE openctf; CREATE DATABASE openctf_tests;"

bash deploy