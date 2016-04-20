OpenCTF
======

Installation
------

You'll need [Vagrant](https://www.vagrantup.com/) to set up OpenCTF. Make sure Vagrant is working correctly from the command line before you continue.

To set up the server, clone this repository, and `vagrant up` from it. The setup script should automatically begin the installation process. After the installation is complete, use `vagrant ssh` to log into the box. All the server files will be available at `/vagrant`.

In order to deploy the server, run `deploy` (from anywhere). Then you can view the site at `http://localhost:8080`.