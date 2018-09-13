// mostly referenced https://github.com/sean3z/rocket-diesel-rest-api-example/blob/master/src/db.rs

use std::ops::Deref;

use diesel::{
    mysql::MysqlConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use rocket::{
    http::Status,
    request::{self, FromRequest},
    Outcome, Request, State,
};

type ManagedMysqlConnection = ConnectionManager<MysqlConnection>;
pub type MysqlPool = Pool<ManagedMysqlConnection>;
pub struct Connection(pub PooledConnection<ManagedMysqlConnection>);

pub fn connect(database_url: impl AsRef<str>) -> MysqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url.as_ref());
    Pool::new(manager).expect("Failed to create the pool.")
}

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<MysqlPool>>()?;
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
