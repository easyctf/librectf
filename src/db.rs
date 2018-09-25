use std::ops::Deref;

use diesel::prelude::MysqlConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;
use rocket::{
    http::Status,
    request::{self, FromRequest, Request, State},
    Outcome,
};

pub struct Connection(PooledConnection<ConnectionManager<MysqlConnection>>);

pub fn establish_connection(
    database_url: impl AsRef<str>,
) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url.as_ref());
    Pool::new(manager).expect("Failed to create pool.")
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Connection, ()> {
        let pool = request.guard::<State<Pool<ConnectionManager<MysqlConnection>>>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
