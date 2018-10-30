extern crate actix_web;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

mod config;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::PathBuf;
use std::str::FromStr;

use actix_web::{fs::NamedFile, error::ErrorForbidden, http::Method, server, App, HttpRequest};
use failure::Error;
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
            .resource("/public/{tail:.*}", |r| r.method(Method::GET).f(public))
            .resource("/private/{tail:.*}", |r| r.method(Method::GET).f(private))
    }).bind(addr)
    .map(|server| server.run())?;
    Ok(())
}
