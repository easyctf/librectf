use diesel::{mysql::MysqlConnection, r2d2::{Pool, PooledConnection, ConnectionManager}};

type ManagedMysqlConnection = ConnectionManager<MysqlConnection>;
pub type MysqlPool = Pool<ManagedMysqlConnection>;
pub struct Connection(pub PooledConnection<ManagedMysqlConnection>);

pub fn connect(database_url: impl AsRef<str>) -> MysqlPool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url.as_ref());
    Pool::new(manager).expect("Failed to create the pool.")
}

