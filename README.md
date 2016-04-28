OpenCTF
======

[![Slack](http://slack.easyctf.com/badge.svg)](http://slack.easyctf.com)
[![Build Status](https://travis-ci.org/EasyCTF/OpenCTF.svg?branch=master)](https://travis-ci.org/EasyCTF/OpenCTF)

[![Online Demo](docs/screenshot1.png)](https://openctf.easyctf.com/)

Demo
------

A demo copy of this platform is up at [OpenCTF](https://openctf.easyctf.com/). The server is running a cron job that resets the site every 2 hours. Report issues [here](https://github.com/EasyCTF/OpenCTF/issues).

Installation
------

You'll need [Vagrant](https://www.vagrantup.com/) to set up OpenCTF. Make sure Vagrant is working correctly from the command line before you continue.

To set up the server, clone this repository, and `vagrant up` from it. The setup script should automatically begin the installation process. After the installation is complete, use `vagrant ssh` to log into the box. All the server files will be available at `/vagrant`.

You need to provide the following environmental variables:

  - `SQLALCHEMY_DATABASE_URI`: The MySQL URL for your database. It looks like: `mysql://user:password@host:port/database`.
  - `MAILGUN_URL`: The Mailgun API URL that the application must connect to.
  - `MAILGUN_KEY`: Your secret API key.
  - `ADMIN_EMAIL`: An email in the format `John Doe <john@doe.com>`. This will be your "from" address.

Instead of setting environment variables, you can also place a `.env` file in `/vagrant/server`. The `config.py` will read the variables from that file instead, but will not set the environmental variables. Here is an example demonstrating the format required for the `.env` file.

    SQLALCHEMY_DATABASE_URI=mysql://root:i_hate_passwords@localhost/openctf
    MAILGUN_URL=https://api.mailgun.net/v3/example.com
    MAILGUN_KEY=key-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
    ADMIN_EMAIL=OpenCTF Administrator <admin@openctf.com>

In order to deploy the server, run `deploy` (from anywhere). Then you can view the site at `http://localhost:8080`.
