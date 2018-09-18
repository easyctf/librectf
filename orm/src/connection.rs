use url::Url;

use backend;
use IntoEntities;
use Query;

pub trait ConnectionPoolExt {}

pub enum ConnectionPool {
    #[cfg(feature = "mysql")]
    Mysql(backend::mysql::MysqlConnectionPool),
}

impl ConnectionPool {
    // TODO: figure out logging
    pub fn from(url: impl AsRef<str>) -> Result<Self, ()> {
        let parsed = Url::parse(url.as_ref()).map_err(|_| ())?;
        match parsed.scheme() {
            #[cfg(feature = "mysql")]
            "mysql" => Ok(ConnectionPool::Mysql(
                backend::mysql::MysqlConnectionPool::new(url),
            )),
            _ => Err(()),
        }
    }

    pub fn query<T>(&self, ent: impl IntoEntities<T>) -> Query<T> {
        Query::new(&self, ent)
    }

    pub fn commit(&self) {
        
    }
}
