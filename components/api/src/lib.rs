#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
extern crate comrak;
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
extern crate regex;
extern crate serde;
extern crate structopt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod api;
mod chal;
mod db;
mod routes;
mod scoreboard;
mod team;
mod user;

use actix_web::App;
use failure::Error;

use core::State;
use db::DbConn;

pub fn app(state: State) -> Result<App<State>, Error> {
    Ok(routes::router(state))
}
