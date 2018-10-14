use std::ops::Deref;

use actix_web::{FromRequest, HttpRequest};
use diesel::MysqlConnection;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;

use super::errors::WebError;
use super::State;

pub struct Connection(pub PooledConnection<ConnectionManager<MysqlConnection>>);

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest<State> for Connection {
    type Config = ();
    type Result = Result<Self, WebError>;

    #[inline]
    fn from_request(req: &HttpRequest<State>, _: &Self::Config) -> Self::Result {
        req.state().get_connection()
    }
}
