mod db;
mod errors;
mod state;
mod user;

use actix_web::{http::Method, server, App};

use self::db::Connection as DbConn;
use self::state::State;
use db::establish_connection;
use errors::AddressBindError;
use Config;
use Error;

const POST: Method = Method::POST;

fn app(config: &Config) -> App<State> {
    let pool = establish_connection(&config.database_url);

    let app = App::with_state(State { pool }).prefix("/api/v1");
    {
        use self::user::*;
        app.resource("/user/login", |r| r.method(POST).f(login))
            .resource("/user/register", |r| r.method(POST).with(register))
    }
}

pub fn run(config: Config) -> Result<(), Error> {
    server::new(move || app(&config))
        .bind("127.0.0.1:8000")
        .map_err(|err| AddressBindError(err).into())
        .map(|server| server.run())
}
