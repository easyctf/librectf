use std::ops::Deref;

use diesel::prelude::MysqlConnection;
use r2d2::{Pool, PooledConnection};
use r2d2_diesel::ConnectionManager;

pub struct Connection(PooledConnection<ConnectionManager<MysqlConnection>>);

pub fn establish_connection(
    database_url: impl AsRef<str>,
) -> Pool<ConnectionManager<MysqlConnection>> {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url.as_ref());
    Pool::new(manager).expect("Failed to create pool.")
}

impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
