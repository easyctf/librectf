LibreCTF
========

[![](https://travis-ci.org/easyctf/librectf.svg?branch=develop)](https://travis-ci.org/easyctf/librectf)
![](https://tokei.rs/b1/github/easyctf/librectf)
[![GitHub version](https://badge.fury.io/gh/easyctf%2Flibrectf.svg)](https://badge.fury.io/gh/easyctf%2Flibrectf)
[![GitHub issues](https://img.shields.io/github/issues/easyctf/librectf.svg)](https://github.com/easyctf/librectf/issues)
[![GitHub forks](https://img.shields.io/github/forks/easyctf/librectf.svg)](https://github.com/easyctf/librectf/network)
[![GitHub stars](https://img.shields.io/github/stars/easyctf/librectf.svg)](https://github.com/easyctf/librectf/stargazers)
[![Rawsec's CyberSecurity Inventory](https://inventory.rawsec.ml/img/badges/Rawsec-inventoried-FF5050_flat.svg)](https://inventory.rawsec.ml/ctf_platforms.html#LibreCTF)

[Documentation](http://easyctf.github.io/librectf/)

LibreCTF is a framework for running CTF (capture-the-flag) competitions. Formerly, this project was known as OpenCTF, named after the platform used to run EasyCTF, but has since been [rewritten into Rust](https://github.com/ansuz/RIIR). As such, it's made with performance in mind, while also aiming to be as flexible as possible.

The recommended method to running this platform is through a Docker container that will be built with releases. The reasoning behind this is because some parts of the platform is required to be run as root: it has a dependency on nsjail, a sandboxing utility that takes advantage of kernel namespacing and other techniques which require elevated permissions. Additionally, this platform has many moving parts which may break if you're not careful! Therefore, it's _highly_ discouraged to run the platform in an environment other than the provided one unless you know what you are doing.

Status
------

LibreCTF is currently under development. Expect to see changes in the coming weeks!

Roadmap
-------

See the [milestone](https://github.com/easyctf/librectf/milestone/3) for the 1.0 release for the roadmap.

License
-------

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
