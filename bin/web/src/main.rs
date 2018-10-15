#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate jsonwebtoken;
#[macro_use]
extern crate log;
#[macro_use]
extern crate openctf_core;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate structopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod base;
mod config;
mod db;
mod errors;
mod state;
mod team;
mod user;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use actix_web::server;
use openctf_core::establish_connection;
use structopt::StructOpt;

use config::WebConfig;
use db::Connection as DbConn;
use errors::WebError;
use state::State;

#[derive(Debug, StructOpt)]
pub struct WebCommand {
    #[structopt(flatten)]
    config: WebConfig,
}

impl WebCommand {
    pub fn run(&self) -> Result<(), WebError> {
        run(self.config.clone())
    }
}

pub fn run(config: WebConfig) -> Result<(), WebError> {
    let pool = establish_connection(&config.database_url);
    let secret_key = config.secret_key.clone().into_bytes();
    let state = State::new(secret_key, pool);

    let addr = SocketAddrV4::new(Ipv4Addr::from_str(&config.bind_host)?, config.bind_port);

    server::new(move || {
        vec![
            base::app(state.clone()),
            team::app(state.clone()),
            user::app(state.clone()),
        ]
    }).bind(addr)
    .map_err(|err| errors::AddressBindError(err).into())
    .map(|server| {
        error!("starting the server...");
        println!("{:?}", ::std::env::var("RUST_LOG"));
        server.run()
    })
}

fn main() -> Result<(), WebError> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn");
    env_logger::Builder::from_env(env).init();

    let opt = WebCommand::from_args();
    opt.run()
}
