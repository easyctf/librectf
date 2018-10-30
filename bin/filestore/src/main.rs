// based on https://github.com/actix/examples/blob/master/multipart/src/main.rs

extern crate actix_web;
extern crate env_logger;
extern crate failure;
extern crate futures;
#[macro_use]
extern crate log;
extern crate sha2;
#[macro_use]
extern crate structopt;

mod config;

use std::fs::File;
use std::io::Write;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::str::FromStr;

use actix_web::{
    dev::Payload,
    error::{
        ErrorBadRequest, ErrorForbidden, ErrorInternalServerError, MultipartError, PayloadError,
    },
    fs::NamedFile,
    http::Method,
    multipart::{Field, MultipartItem},
    server, App, FutureResponse, HttpMessage, HttpRequest, HttpResponse,
};
use failure::Error;
use futures::{future, Future, Stream};
use sha2::{Digest, Sha256};
use structopt::StructOpt;

use config::FsConfig;

#[derive(Debug, StructOpt)]
struct FsService {
    #[structopt(flatten)]
    config: FsConfig,
}

#[derive(Clone)]
struct State(pub FsConfig);

impl State {
    fn new(config: &FsConfig) -> Self {
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

fn save_file(
    storage_dir: PathBuf,
    field: Field<Payload>,
) -> Box<Future<Item = String, Error = actix_web::Error>> {
    let filename = match field
        .content_disposition()
        .and_then(|cd| cd.get_name().map(|val| val.to_owned()))
    {
        Some(name) => name,
        None => return Box::new(future::err(ErrorBadRequest("No filename specified."))),
    };

    let path = storage_dir.join(PathBuf::from(filename));
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(err) => {
            error!("Error creating file: {:?}", err);
            return Box::new(future::err(ErrorInternalServerError(err)));
        }
    };

    Box::new(
        field
            .fold(Sha256::new(), move |mut hasher, bytes| {
                let result = file
                    .write_all(bytes.as_ref())
                    .map(|_| {
                        hasher.input(bytes.as_ref());
                        hasher
                    }).map_err(|err| {
                        error!("Failed to write to file: {:?}", err);
                        MultipartError::Payload(PayloadError::Io(err))
                    });;
                future::result(result)
            }).map_err(|err| {
                error!("Multipart error: {:?}", err);
                ErrorInternalServerError(err)
            }).map(|hasher| format!("{:x}", hasher.result())),
    )
}

fn handle_multipart(
    storage_dir: PathBuf,
    item: MultipartItem<Payload>,
) -> Box<Stream<Item = String, Error = actix_web::Error>> {
    match item {
        MultipartItem::Field(field) => Box::new(save_file(storage_dir, field).into_stream()),
        MultipartItem::Nested(nested) => Box::new(
            nested
                .map_err(ErrorInternalServerError)
                .map(move |item| handle_multipart(storage_dir.clone(), item))
                .flatten(),
        ),
    }
}

fn upload(req: &HttpRequest<State>) -> FutureResponse<HttpResponse> {
    let state = req.state();
    let headers = req.headers();
    let storage_dir = state.0.storage_dir.clone();

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
            .map(move |item| handle_multipart(storage_dir.clone(), item))
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

fn main() -> Result<(), Error> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn");
    env_logger::Builder::from_env(env).init();

    let config = FsConfig::from_args();
    let addr = SocketAddrV4::new(
        Ipv4Addr::from_str(&config.bind_host).unwrap(),
        config.bind_port,
    );

    server::new(move || {
        let state = State::new(&config);
        App::with_state(state)
            .resource("/upload", |r| r.method(Method::POST).f(upload))
            .resource("/public/{tail:.*}", |r| r.method(Method::GET).f(public))
            .resource("/private/{tail:.*}", |r| r.method(Method::GET).f(private))
    }).bind(addr)
    .map(|server| server.run())?;
    Ok(())
}
