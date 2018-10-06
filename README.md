OpenCTF
=======

[![](https://travis-ci.org/easyctf/openctf.svg?branch=develop)](https://travis-ci.org/easyctf/openctf)
![](https://tokei.rs/b1/github/easyctf/openctf)

[Documentation](http://easyctf.github.io/openctf/)

OpenCTF is a framework for running CTF (capture-the-flag) competitions. The name OpenCTF comes from the platform used to run EasyCTF, but has since been [rewritten into Rust](https://github.com/ansuz/RIIR). As such, it's made with performance in mind, while also aiming to be as flexible as possible.

The recommended method to running this platform is through a Docker container that will be built with releases. The reasoning behind this is because the platform is required to be run as root: it has a dependency on nsjail, a sandboxing utility that takes advantage of kernel namespacing and other techniques which require elevated permissions. Because of this dependency, it's _highly_ discouraged to run the platform in an environment other than the provided one unless you know what you are doing.

Status
------

OpenCTF is currently under development. Expect to see changes in the coming weeks!

Roadmap
-------

See the [milestone](https://github.com/easyctf/openctf/milestone/3) for the 1.0 release for the roadmap.

Contact
-------

Author: Michael Zhang

License: MIT
