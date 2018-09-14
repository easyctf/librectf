use url::Url;

use backend;
use {AsQuery, Query};

pub trait ConnectionPoolExt {
    fn run(&self, impl AsQuery);
}

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

    pub fn run(&self, query: impl AsQuery) {
        // send query to the server
        // backend specific
        match self {
            #[cfg(feature = "mysql")]
            ConnectionPool::Mysql(backend) => backend.run(query),
        }
    }
}
