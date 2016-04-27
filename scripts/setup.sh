#!/bin/bash

MYSQL_ROOT_PASSWORD="i_hate_passwords"

echo "Updating system..."
apt-get -y update
apt-get -y upgrade

echo "Preparing for MySQL installation..."
sudo debconf-set-selections <<< "mysql-server mysql-server/root_password password $MYSQL_ROOT_PASSWORD"
sudo debconf-set-selections <<< "mysql-server mysql-server/root_password_again password $MYSQL_ROOT_PASSWORD"

echo "Installing dependencies..."
apt-get -y install python dos2unix
apt-get -y install python-pip libjpeg-dev
apt-get -y install python-dev libmysqlclient-dev
apt-get -y install nginx
apt-get -y install mysql-server memcached
apt-get -y install tmux

echo "Installing pip dependencies..."
pip install -r /vagrant/scripts/requirements.txt

echo "PATH=$PATH:/vagrant" >> /etc/profile
source /etc/profile
cp /vagrant/ctf.nginx /etc/nginx/sites-enabled/ctf
rm /etc/nginx/sites-*/default

sudo service nginx restart

mysql -u root -p"$MYSQL_ROOT_PASSWORD" -e "CREATE DATABASE openctf;"