#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate jsonwebtoken;
#[macro_use]
extern crate log;
extern crate core;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate redis;
#[macro_use]
extern crate structopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod api;
mod chal;
mod config;
mod db;
mod routes;
mod scoreboard;
mod state;
mod task;
mod team;
mod user;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use actix_web::server;
use core::establish_connection;

use api::APIMiddleware;
pub use config::Config;
use db::DbConn;
use state::State;

pub fn run(config: Config) {
    let addr = SocketAddrV4::new(
        Ipv4Addr::from_str(&config.bind_host).unwrap(),
        config.bind_port,
    );

    let pool = establish_connection(&config.database_url);
    let state = State::new(config.secret_key.clone().into_bytes(), pool);

    server::new(move || routes::router(state.clone()))
        .bind(addr)
        .map(|server| server.run())
        .unwrap();
}
