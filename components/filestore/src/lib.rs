// based on https://github.com/actix/examples/blob/master/multipart/src/main.rs
// TODO: mime type on output
// TODO: prefix/suffix uploading

extern crate actix_web;
extern crate core;
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate serde;
extern crate sha2;
#[macro_use]
extern crate serde_derive;
extern crate structopt;
extern crate tempfile;

mod config;
mod util;

use std::path::PathBuf;

use actix_web::{
    error::{ErrorForbidden, ErrorInternalServerError},
    fs::NamedFile,
    App, FromRequest, FutureResponse, HttpMessage, HttpRequest, HttpResponse,
};
use core::State;
use failure::Error;
use futures::{future, Future, Stream};

pub use config::Config;
use util::handle_multipart;

pub fn app(state: State) -> Result<App<State>, Error> {
    let app = App::with_state(state)
        .resource("/upload/public", |r| r.post().f(upload))
        .resource("/upload/private", |r| r.post().f(upload))
        .resource("/public/{tail:.*}", |r| r.get().f(public))
        .resource("/private/{tail:.*}", |r| r.get().f(private));
    Ok(app)
}

fn private(req: &HttpRequest<State>) -> actix_web::Result<NamedFile> {
    let headers = req.headers();
    let cfg = Config::from_request(&req, &())?;

    match headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(|token| token == cfg.pull_password)
    {
        Some(true) => (),
        _ => return Err(ErrorForbidden("Not authorized.")),
    }

    let tail: PathBuf = req.match_info().query("tail")?;

    let path = cfg.storage_dir.join("private").join(tail);
    Ok(NamedFile::open(path)?)
}

fn public(req: &HttpRequest<State>) -> actix_web::Result<NamedFile> {
    let tail: PathBuf = req.match_info().query("tail")?;
    info!("tail = {}", tail.display());
    let cfg = Config::from_request(&req, &())?;

    let mut path = cfg.storage_dir.clone();
    path.push("public");
    path.push(tail);

    info!("Received public request: {:?}", path);
    Ok(NamedFile::open(path)?)
}

#[derive(Serialize, Deserialize)]
struct UploadOptions {
    prefix: Option<String>,
    suffix: Option<String>,
}

fn upload(req: &HttpRequest<State>) -> FutureResponse<HttpResponse> {
    let headers = req.headers();
    let cfg = match Config::from_request(&req, &()) {
        Ok(cfg) => cfg,
        Err(err) => {
            error!("TODO: decipher this {}", err);
            return Box::new(future::err(err.into()));
        }
    };

    let storage_dir = cfg.storage_dir.clone();
    let private = req.uri().path().ends_with("private");

    match headers
        .get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(|token| token == cfg.push_password)
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
                HttpResponse::Ok().json(result)
            }).map_err(|err| {
                error!("Error during upload: {:?}", err);
                err
            }),
    )
}
