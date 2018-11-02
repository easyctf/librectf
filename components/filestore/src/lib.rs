// based on https://github.com/actix/examples/blob/master/multipart/src/main.rs
// TODO: mime type on output

extern crate actix_web;
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate serde;
extern crate sha2;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
extern crate tempfile;

mod config;
mod util;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::str::FromStr;

use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError},
    fs::NamedFile,
    http::Method,
    server, App, FutureResponse, HttpMessage, HttpRequest, HttpResponse,
};
use failure::Error;
use futures::{future, Future, Stream};
use sha2::Digest;
use structopt::StructOpt;

pub use config::Config;
use util::handle_multipart;

#[derive(Debug, StructOpt)]
pub struct FilestoreCommand {
    #[structopt(flatten)]
    config: Config,
}

impl FilestoreCommand {
    pub fn run(&self) -> Result<(), Error> {
        let addr = SocketAddrV4::new(
            Ipv4Addr::from_str(&self.config.bind_host).unwrap(),
            self.config.bind_port,
        );
        let config = self.config.clone();

        server::new(move || {
            let state = State::new(&config.clone());
            App::with_state(state)
                .resource("/upload/public", |r| r.method(Method::POST).f(upload))
                .resource("/upload/private", |r| r.method(Method::POST).f(upload))
                .resource("/public/{tail:.*}", |r| r.method(Method::GET).f(public))
                .resource("/private/{tail:.*}", |r| r.method(Method::GET).f(private))
        }).bind(addr)
        .map(|server| server.run())
        .unwrap();

        Ok(())
    }
}

#[derive(Clone)]
struct State(pub Config);

impl State {
    fn new(config: &Config) -> Self {
        State(config.clone())
    }
}

fn private(req: &HttpRequest<State>) -> actix_web::Result<NamedFile> {
    let state = req.state();
    let headers = req.headers();

    match headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(|token| token == state.0.pull_password)
    {
        Some(true) => (),
        _ => return Err(ErrorForbidden("Not authorized.")),
    }

    let tail: PathBuf = req.match_info().query("tail")?;

    let mut path = state.0.storage_dir.clone();
    path.push("private");
    path.push(tail);

    Ok(NamedFile::open(path)?)
}

fn public(req: &HttpRequest<State>) -> actix_web::Result<NamedFile> {
    let state = req.state();
    let tail: PathBuf = req.match_info().query("tail")?;

    let mut path = state.0.storage_dir.clone();
    path.push("public");
    path.push(tail);
    error!("path: {:?}", path);

    Ok(NamedFile::open(path)?)
}

#[derive(Serialize, Deserialize)]
struct UploadOptions {
    prefix: Option<String>,
    suffix: Option<String>,
}

fn upload(req: &HttpRequest<State>) -> FutureResponse<HttpResponse> {
    let state = req.state();
    let headers = req.headers();
    let storage_dir = state.0.storage_dir.clone();
    let private = req.uri().path().ends_with("private");

    match headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(|token| token == state.0.push_password)
    {
        Some(true) => (),
        _ => return Box::new(future::err(ErrorForbidden("Not authorized."))),
    }

    Box::new(
        req.multipart()
            .map_err(ErrorInternalServerError)
            .map(move |item| handle_multipart(private, storage_dir.clone(), item))
            .flatten()
            .collect()
            .map(|result| {
                println!("{:?}", result);
                HttpResponse::Ok().json(result)
            }).map_err(|err| {
                error!("Error during upload: {:?}", err);
                err
            }),
    )
}
