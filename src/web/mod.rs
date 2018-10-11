mod config;
mod db;
mod errors;
mod state;
mod user;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use actix_web::{self, http::Method, server, App, HttpRequest, HttpResponse, Json, Responder};
use serde::Serialize;

pub use self::config::WebConfig;
use self::db::Connection as DbConn;
use self::state::State;
use db::establish_connection;
use errors::AddressBindError;
use Error;

const POST: Method = Method::POST;

fn app(config: &WebConfig) -> App<State> {
    let pool = establish_connection(&config.database_url);
    let secret_key = config.secret_key.clone().into_bytes();

    let app = App::with_state(State { secret_key, pool }).prefix("/api/v1");

    let app = {
        use self::user::*;
        app.resource("/user/login", |r| r.method(POST).with(login))
            .resource("/user/register", |r| r.method(POST).with(register))
    };
    app
}

pub fn run(config: WebConfig) -> Result<(), Error> {
    let addr = SocketAddrV4::new(Ipv4Addr::from_str(&config.bind_host)?, config.bind_port);
    server::new(move || app(&config))
        .bind(addr)
        .map_err(|err| AddressBindError(err).into())
        .map(|server| server.run())
}
