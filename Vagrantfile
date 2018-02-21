#!/usr/bin/ruby

Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/xenial64"
  config.vm.network "forwarded_port", guest: 80, host: 7000
  config.vm.network "forwarded_port", guest: 8000, host: 8000

  config.vm.synced_folder "../problems", "/problems"
  config.vm.provider "virtualbox" do |vb|
  end

  # config.vm.provision "shell", path: "provision.sh"
end
