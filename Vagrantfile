# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/trusty64"
  # config.vm.box_check_update = false

  config.vm.network :forwarded_port, guest: 25, host: 25, auto_correct: true
  config.vm.network :forwarded_port, guest: 80, host: 8080, auto_correct: true
  config.vm.network :forwarded_port, guest: 8000, host: 8000, auto_correct: true
  config.vm.network :forwarded_port, guest: 27017, host: 27017, auto_correct: true

  config.vm.synced_folder "web", "/srv/http/ctf"
  config.vm.synced_folder ".", "/home/vagrant", :nfs => true

  config.vm.provision :shell, :path => "setup.sh"
  config.ssh.forward_agent = true

  config.vm.provider "virtualbox" do |vb|
    vb.customize ["modifyvm", :id, "--memory", "2048"]
  end
  config.vm.provider :virtualbox do |vb|
    vb.customize ["modifyvm", :id, "--natdnshostresolver1", "on"]
    vb.customize ["modifyvm", :id, "--natdnsproxy1", "on"]
  end
end
