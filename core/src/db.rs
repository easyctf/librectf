use std::str::FromStr;

use diesel::r2d2::ConnectionManager;
use r2d2::Pool;
use url::{ParseError, Url};

#[cfg(feature = "mysql")]
use diesel::mysql::MysqlConnection;

#[cfg(feature = "postgres")]
use diesel::pg::PgConnection;

#[cfg(feature = "sqlite")]
use diesel::sqlite::SqliteConnection;

use crate::Error;

pub enum DbPool {
    #[cfg(feature = "mysql")]
    Mysql(Pool<ConnectionManager<MysqlConnection>>),

    #[cfg(feature = "postgres")]
    Postgres(Pool<ConnectionManager<PgConnection>>),

    #[cfg(feature = "sqlite")]
    Sqlite(Pool<ConnectionManager<SqliteConnection>>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DatabaseUri {
    #[cfg(feature = "mysql")]
    Mysql(String),

    #[cfg(feature = "postgres")]
    Postgres(String),

    #[cfg(feature = "sqlite")]
    Sqlite(String),
}

impl DatabaseUri {
    pub fn establish_connection(&self) -> Result<DbPool, Error> {
        match self {
            #[cfg(feature = "mysql")]
            DatabaseUri::Mysql(url) => {
                let manager = ConnectionManager::<MysqlConnection>::new(url.as_ref());
                Ok(DbPool::Mysql(Pool::new(manager)?))
            }
            #[cfg(feature = "postgres")]
            DatabaseUri::Postgres(url) => {
                let manager = ConnectionManager::<PgConnection>::new(url.as_ref());
                Ok(DbPool::Postgres(Pool::new(manager)?))
            }
            #[cfg(feature = "sqlite")]
            DatabaseUri::Sqlite(url) => {
                let manager = ConnectionManager::<SqliteConnection>::new(url.as_ref());
                Ok(DbPool::Sqlite(Pool::new(manager)?))
            }
        }
    }
}

impl FromStr for DatabaseUri {
    type Err = ParseError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(string)?;
        match url.scheme() {
            #[cfg(feature = "mysql")]
            "mysql" => Ok(DatabaseUri::Mysql(url.as_str().to_owned())),
            #[cfg(feature = "postgres")]
            "postgres" => Ok(DatabaseUri::Postgres(url.as_str().to_owned())),
            #[cfg(feature = "sqlite")]
            "sqlite" => Ok(DatabaseUri::Sqlite(url.path().to_owned())),
            // TODO: an actual error
            _ => Err(ParseError::IdnaError),
        }
    }
}
