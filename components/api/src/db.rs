use std::ops::Deref;

use actix_web::{FromRequest, HttpRequest};
use core::db::Connection;

use super::State;

pub struct DbConn(Connection);

impl Deref for DbConn {
    type Target = <Connection as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl FromRequest<State> for DbConn {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<State>, _: &Self::Config) -> Self::Result {
        req.state().get_connection().map(|conn| DbConn(conn))
    }
}
