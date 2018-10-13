# OpenCTF

OpenCTF is an open-source CTF platform designed to be flexible and run under lots of different environments.

## Components

There are several components that comprise OpenCTF. For a small setup, it should suffice to simply run them all in the same box. But if you're planning to run a larger-scale CTF, it may be wise to deploy on multiple servers. These components can all live on different servers and talk to each other over the network, for good horizontal scaling.

- **Gateway/Proxy server**: This is the main frontend of the CTF and serves two purposes: to serve as the gateway to the API server and filestore servers, and to serve the static frontend.
- **API Server**: This is the backbone of the CTF. It connects all of the other services and exposes an interface of endpoints to the web.
- **Static Filestore**: This server actually has two endpoints: one internal-facing (can be protected by a password or SSH) endpoint to receive files, and a public-facing one that serves static files. In reality, this can live on the same server as the gateway, since it also uses Nginx.
- **Task queue**: This server consumes items from the task queue and executes them. Some examples of tasks that are executed are: sending emails, running sandboxed applications, generating files, precomputing the scoreboard.

And then some more common services:

- **SQL Database**: Currently, OpenCTF only supports MySQL (and by extension, MariaDB). Support for other servers is planned, but not likely to be implemented for a while. PRs for porting the migrations into other flavors of SQL are welcome!
- **Redis database (optional)**: This serves as both a cache server and PubSub server. It is not be used for any information that must not be lost, so it's ok to lose this. Without a Redis backend, a built-in hashmap-based implementation will be used.

Of course, it goes without saying that these services should all run on Linux servers. Windows is not and will never be supported for the server side.
