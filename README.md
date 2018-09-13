OpenCTF
=======

OpenCTF is a framework for running CTF (capture-the-flag) competitions. The name OpenCTF comes from the platform used to run EasyCTF, but has since been [rewritten into Rust](https://github.com/ansuz/RIIR). As such, it's made with performance in mind, while also aiming to be as flexible as possible.

Status
------

OpenCTF is currently under development. Expect to see changes in the coming weeks!

Notes
-----

As it stands, OpenCTF only supports **MySQL**, or MariaDB (which is a drop-in replacement). This is due to `diesel` requiring migrations to be written in backend-dependent SQL files. Also I don't want to write the same `CREATE TABLE` statements a million times.

Contact
-------

Author: Michael Zhang

License: MIT
