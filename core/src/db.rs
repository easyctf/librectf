use diesel::prelude::MysqlConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection(database_url: impl AsRef<str>) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url.as_ref());
    r2d2::Pool::new(manager).expect("Failed to create pool.")
}
