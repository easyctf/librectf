mod state;
mod user;

use actix_web::{http, server, App};

use self::state::State;
use errors::AddressBindError;
use Config;
use Error;

fn app(config: &Config) -> App<State> {
    App::with_state(State {})
        .prefix("/api/v1")
        .resource("/user/register", |r| {
            r.method(http::Method::POST).f(user::register)
        })
}

pub fn run(config: Config) -> Result<(), Error> {
    server::new(move || app(&config))
        .bind("127.0.0.1:8000")
        .map_err(|err| AddressBindError(err).into())
        .map(|server| server.run())
}
