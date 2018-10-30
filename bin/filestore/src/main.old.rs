extern crate bytes;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate http;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate structopt;

mod config;

use std::fs::File;
use std::io::{self, Cursor, Read};
use std::path::PathBuf;

use bytes::{Bytes, IntoBuf};
use failure::{Compat, Error};
use futures::{future, Async, Future, Poll, Stream};
use hyper::{
    body::{Body, Chunk, Payload},
    service::Service,
    Request, Response, Server,
};
use structopt::StructOpt;

use config::FsConfig;

#[derive(Debug, StructOpt)]
struct FsService {
    #[structopt(flatten)]
    config: FsConfig,
}

struct StreamingBody(pub File);

impl Payload for StreamingBody {
    type Data = Chunk;
    type Error = io::Error;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        let mut buf = vec![0; 1024];
        self.0.read(&mut buf)?;
        Ok(Async::Ready(Some(Chunk::from(buf))))
    }
}

enum Bodies {
    Static(Body),
    Stream(StreamingBody),
}

impl Payload for Bodies {
    type Data = Cursor<Bytes>;
    type Error = Compat<Error>;

    fn poll_data(&mut self) -> Poll<Option<Self::Data>, Self::Error> {
        match self {
            Bodies::Static(body) => body
                .poll_data()
                .map(|chunkasync| {
                    chunkasync.map(|chunkopt| chunkopt.map(|chunk| chunk.into_bytes().into_buf()))
                }).map_err(|err| format_err!("{}", err).compat()),
            Bodies::Stream(body) => body
                .poll_data()
                .map(|chunkasync| {
                    chunkasync.map(|chunkopt| chunkopt.map(|chunk| chunk.into_bytes().into_buf()))
                }).map_err(|err| format_err!("{}", err).compat()),
        }
    }
}

#[derive(Deserialize)]
struct UploadOpt {
    private: bool,
}

impl Service for FsService {
    type ReqBody = Body;
    type ResBody = Bodies;
    type Error = String;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, req: Request<Self::ReqBody>) -> Self::Future {
        let (parts, body) = req.into_parts();
        Box::new(
            body.concat2()
                .map_err(|err| format!("error: {}", err))
                .map(move |chunks| {
                    let uri = parts.uri.path();
                    if uri.starts_with("/public") || uri.starts_with("private") {
                        let path = PathBuf::from(uri);
                        let file = File::open(path).unwrap();
                        return Response::new(Bodies::Stream(StreamingBody(file)));
                    } else if uri == "/upload" {
                        let payload = chunks.into_bytes();
                        let data = serde_json::from_slice::<UploadOpt>(&payload).unwrap();

                    }
                    Response::new(Bodies::Static(Body::from("lo")))
                }),
        )
    }
}

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn");
    env_logger::Builder::from_env(env).init();

    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| future::ok::<_, hyper::Error>(FsService::from_args()))
        .map_err(|err| error!("Hyper error: {}", err));
    hyper::rt::run(server);
}
