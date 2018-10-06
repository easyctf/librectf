mod db;
mod config;
mod errors;
mod state;
mod user;

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

    let app = App::with_state(State { pool }).prefix("/api/v1");
    {
        use self::user::*;
        app.resource("/user/login", |r| r.method(POST).f(login))
            .resource("/user/register", |r| r.method(POST).with(register))
    }
}

pub fn run(config: WebConfig) -> Result<(), Error> {
    server::new(move || app(&config))
        .bind("127.0.0.1:8000")
        .map_err(|err| AddressBindError(err).into())
        .map(|server| server.run())
}

pub enum JsonResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> JsonResult<T, E> {
    pub fn ok(v: impl AsRef<str>) -> JsonResult<String, E> {
        JsonResult::Ok(v.as_ref().to_owned())
    }

    pub fn err(v: impl AsRef<str>) -> JsonResult<T, String> {
        JsonResult::Err(v.as_ref().to_owned())
    }
}

impl<T: Serialize, E: Serialize> Responder for JsonResult<T, E> {
    type Item = HttpResponse;
    type Error = actix_web::Error;

    fn respond_to<S: 'static>(self, req: &HttpRequest<S>) -> Result<Self::Item, Self::Error> {
        match self {
            JsonResult::Ok(t) => Responder::respond_to(Json(t), req),
            JsonResult::Err(e) => Responder::respond_to(Json(e), req),
        }
    }
}
