mod config;
mod db;
mod errors;
mod state;
mod team;
mod user;

use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use actix_web::{server, App};

pub use self::config::WebConfig;
use self::db::Connection as DbConn;
use self::state::State;
use db::establish_connection;
use errors::AddressBindError;
use Error;

fn app(config: &WebConfig) -> App<State> {
    let pool = establish_connection(&config.database_url);
    let secret_key = config.secret_key.clone().into_bytes();

    let app = App::with_state(State { secret_key, pool });

    let app = {
        use self::user::*;
        app.resource("/user/login", |r| r.post().with(login))
            .resource("/user/register", |r| r.post().with(register))
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
