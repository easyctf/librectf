use std::io::Write;
use std::str::FromStr;

use diesel::{
    backend::Backend,
    connection::SimpleConnection,
    deserialize::FromSql,
    expression::NonAggregate,
    insertable::{CanInsertInSingleQuery, Insertable},
    prelude::*,
    query_builder::QueryFragment,
    query_source::Table,
    r2d2::ConnectionManager,
    sql_types::HasSqlType,
    RunQueryDsl, SelectableExpression,
};
use r2d2::Pool;
use url::{ParseError, Url};

use crate::models::{NewTeam, NewUser, Team, User};

#[cfg(feature = "mysql")]
mod mysql {
    pub use diesel::mysql::{Mysql, MysqlConnection};
    use diesel::r2d2::{ConnectionManager, PooledConnection};
    pub type MysqlPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;
    #[derive(EmbedMigrations)]
    #[embed_migrations_options(migrations_path = "migrations/mysql")]
    struct _Dummy;
    no_arg_sql_function!(last_insert_id, diesel::sql_types::Integer);
}

#[cfg(feature = "postgres")]
mod postgres {
    pub use diesel::pg::{Pg, PgConnection};
    use diesel::r2d2::{ConnectionManager, PooledConnection};
    pub type PostgresPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
    #[derive(EmbedMigrations)]
    #[embed_migrations_options(migrations_path = "migrations/postgres")]
    struct _Dummy;
}

#[cfg(feature = "sqlite")]
mod sqlite {
    use diesel::r2d2::{ConnectionManager, PooledConnection};
    pub use diesel::sqlite::{Sqlite, SqliteConnection};
    pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;
    #[derive(EmbedMigrations)]
    #[embed_migrations_options(migrations_path = "migrations/sqlite")]
    struct _Dummy;
    no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);
}

use crate::Error;

/// A single connection to a database.
#[allow(missing_docs)]
pub enum DbConn {
    #[cfg(feature = "mysql")]
    Mysql(mysql::MysqlPooledConnection),

    #[cfg(feature = "postgres")]
    Postgres(postgres::PostgresPooledConnection),

    #[cfg(feature = "sqlite")]
    Sqlite(sqlite::SqlitePooledConnection),
}

impl DbConn {
    /// Runs the embedded migrations against the database.
    pub fn run_migrations(&self, out: &mut dyn Write) -> Result<(), Error> {
        match self {
            #[cfg(feature = "mysql")]
            DbConn::Mysql(conn) => mysql::run_with_output(conn, out),

            #[cfg(feature = "postgres")]
            DbConn::Postgres(conn) => postgres::run_with_output(conn, out),

            #[cfg(feature = "sqlite")]
            DbConn::Sqlite(conn) => sqlite::run_with_output(conn, out),
        }
        .map_err(Error::from)
    }
}

/// A database pool.
#[allow(missing_docs)]
pub enum DbPool {
    #[cfg(feature = "mysql")]
    Mysql(Pool<ConnectionManager<self::mysql::MysqlConnection>>),

    #[cfg(feature = "postgres")]
    Postgres(Pool<ConnectionManager<self::postgres::PgConnection>>),

    #[cfg(feature = "sqlite")]
    Sqlite(Pool<ConnectionManager<self::sqlite::SqliteConnection>>),
}

impl DbPool {
    /// Gets a single database connection from the pool.
    pub fn get(&self) -> Result<DbConn, Error> {
        match self {
            #[cfg(feature = "mysql")]
            DbPool::Mysql(conn) => conn.get().map(DbConn::Mysql),

            #[cfg(feature = "postgres")]
            DbPool::Postgres(conn) => conn.get().map(DbConn::Postgres),

            #[cfg(feature = "sqlite")]
            DbPool::Sqlite(conn) => conn.get().map(DbConn::Sqlite),
        }
        .map_err(Error::from)
    }
}

/// A database connection string, for the MySQL, PostgreSQL, or SQLite backends.
///
/// The `FromStr` implementation for this will use the schema (part before the
/// `://`) to determine which one it is.
///
/// - MySQL: `mysql://user:pass@host:port/dbname`
/// - PostgreSQL: `postgres://user:pass@host:port/dbname`
/// - SQLite: `sqlite:///home/user/path/to/db`
///
/// Each of these possible detections is **only** possible if that respective
/// feature has been enabled.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(missing_docs)]
pub enum DatabaseUri {
    #[cfg(feature = "mysql")]
    Mysql(String),

    #[cfg(feature = "postgres")]
    Postgres(String),

    #[cfg(feature = "sqlite")]
    Sqlite(String),
}

impl DatabaseUri {
    /// Opens a connection to the database using the connection type and the
    /// uri given.
    pub fn establish_connection(&self) -> Result<DbPool, Error> {
        match self {
            #[cfg(feature = "mysql")]
            DatabaseUri::Mysql(url) => {
                let manager = ConnectionManager::<self::mysql::MysqlConnection>::new(url.as_ref());
                Ok(DbPool::Mysql(Pool::new(manager)?))
            }

            #[cfg(feature = "postgres")]
            DatabaseUri::Postgres(url) => {
                let manager = ConnectionManager::<self::postgres::PgConnection>::new(url.as_ref());
                Ok(DbPool::Postgres(Pool::new(manager)?))
            }

            #[cfg(feature = "sqlite")]
            DatabaseUri::Sqlite(url) => {
                let manager =
                    ConnectionManager::<self::sqlite::SqliteConnection>::new(url.as_ref());
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

impl SimpleConnection for DbConn {
    fn batch_execute(&self, query: &str) -> Result<(), diesel::result::Error> {
        match self {
            #[cfg(feature = "mysql")]
            DbConn::Mysql(conn) => conn.batch_execute(query),

            #[cfg(feature = "postgres")]
            DbConn::Postgres(conn) => conn.batch_execute(query),

            #[cfg(feature = "sqlite")]
            DbConn::Sqlite(conn) => conn.batch_execute(query),
        }
    }
}

trait UsefulFunctions<B: Backend>: Sized {
    type Backend: Backend;

    fn create_and_return<T, V, R>(&self, target: T, values: V, id: R) -> Result<i32, Error>
    where
        Self::Backend: HasSqlType<R::SqlType>,
        i32: FromSql<R::SqlType, Self::Backend>,
        R: SelectableExpression<T> + NonAggregate + QueryFragment<Self::Backend>,
        T: Table,
        T::FromClause: QueryFragment<Self::Backend>,
        V: Insertable<T>,
        V::Values: QueryFragment<Self::Backend> + CanInsertInSingleQuery<Self::Backend>;
}

#[cfg(feature = "mysql")]
impl UsefulFunctions<mysql::Mysql> for mysql::MysqlPooledConnection {
    type Backend = mysql::Mysql;

    fn create_and_return<T, V, R>(&self, target: T, values: V, _id: R) -> Result<i32, Error>
    where
        Self::Backend: HasSqlType<R::SqlType>,
        i32: FromSql<R::SqlType, Self::Backend>,
        R: SelectableExpression<T> + NonAggregate + QueryFragment<Self::Backend>,
        T: Table,
        T::FromClause: QueryFragment<Self::Backend>,
        V: Insertable<T>,
        V::Values: QueryFragment<Self::Backend> + CanInsertInSingleQuery<Self::Backend>,
    {
        self.transaction(|| {
            diesel::insert_into(target).values(values).execute(self)?;
            diesel::select(mysql::last_insert_id).first(self)
        })
        .map_err(Error::from)
    }
}

#[cfg(feature = "postgres")]
impl UsefulFunctions<postgres::Pg> for postgres::PostgresPooledConnection {
    type Backend = postgres::Pg;

    fn create_and_return<T, V, R>(&self, target: T, values: V, id: R) -> Result<i32, Error>
    where
        Self::Backend: HasSqlType<R::SqlType>,
        i32: FromSql<R::SqlType, Self::Backend>,
        R: SelectableExpression<T> + NonAggregate + QueryFragment<Self::Backend>,
        T: Table,
        T::FromClause: QueryFragment<Self::Backend>,
        V: Insertable<T>,
        V::Values: QueryFragment<Self::Backend> + CanInsertInSingleQuery<Self::Backend>,
    {
        diesel::insert_into(target)
            .values(values)
            .returning(id)
            .get_result(self)
            .map_err(Error::from)
    }
}

#[cfg(feature = "sqlite")]
impl UsefulFunctions<sqlite::Sqlite> for sqlite::SqlitePooledConnection {
    type Backend = sqlite::Sqlite;

    fn create_and_return<T, V, R>(&self, target: T, values: V, _id: R) -> Result<i32, Error>
    where
        Self::Backend: HasSqlType<R::SqlType>,
        i32: FromSql<R::SqlType, Self::Backend>,
        R: SelectableExpression<T> + NonAggregate + QueryFragment<Self::Backend>,
        T: Table,
        T::FromClause: QueryFragment<Self::Backend>,
        V: Insertable<T>,
        V::Values: QueryFragment<Self::Backend> + CanInsertInSingleQuery<Self::Backend>,
    {
        self.transaction(|| {
            diesel::insert_into(target).values(values).execute(self)?;
            diesel::select(sqlite::last_insert_rowid).first(self)
        })
        .map_err(Error::from)
    }
}

impl DbConn {
    /// Tries to fetch the user with the given id.
    pub fn fetch_user_id(&self, id: i32) -> Result<User, Error> {
        use crate::schema::users::dsl;
        let query = dsl::users.filter(dsl::id.eq(id));
        match self {
            #[cfg(feature = "mysql")]
            DbConn::Mysql(conn) => query.first(conn),
            #[cfg(feature = "postgres")]
            DbConn::Postgres(conn) => query.first(conn),
            #[cfg(feature = "sqlite")]
            DbConn::Sqlite(conn) => query.first(conn),
        }
        .map_err(Error::from)
    }

    /// Tries to fetch the user with the given email. (TODO: lookup by username)
    pub fn fetch_user(&self, email: impl AsRef<str>) -> Result<User, Error> {
        use crate::schema::users::dsl;
        let query = dsl::users.filter(dsl::email.eq(email.as_ref()));
        match self {
            #[cfg(feature = "mysql")]
            DbConn::Mysql(conn) => query.first(conn),
            #[cfg(feature = "postgres")]
            DbConn::Postgres(conn) => query.first(conn),
            #[cfg(feature = "sqlite")]
            DbConn::Sqlite(conn) => query.first(conn),
        }
        .map_err(Error::from)
    }

    /// Tries to insert `user` into the database.
    pub fn create_user(&self, user: &NewUser) -> Result<i32, Error> {
        use crate::schema::users;
        match self {
            #[cfg(feature = "mysql")]
            DbConn::Mysql(conn) => conn.create_and_return(users::table, user, users::dsl::id),
            #[cfg(feature = "postgres")]
            DbConn::Postgres(conn) => conn.create_and_return(users::table, user, users::dsl::id),
            #[cfg(feature = "sqlite")]
            DbConn::Sqlite(conn) => conn.create_and_return(users::table, user, users::dsl::id),
        }
        .map_err(Error::from)
    }

    /// Tries to insert `user` into the database.
    pub fn create_team(&self, team: &NewTeam) -> Result<i32, Error> {
        use crate::schema::teams;
        match self {
            #[cfg(feature = "mysql")]
            DbConn::Mysql(conn) => conn.create_and_return(teams::table, team, teams::dsl::id),
            #[cfg(feature = "postgres")]
            DbConn::Postgres(conn) => conn.create_and_return(teams::table, team, teams::dsl::id),
            #[cfg(feature = "sqlite")]
            DbConn::Sqlite(conn) => conn.create_and_return(teams::table, team, teams::dsl::id),
        }
        .map_err(Error::from)
    }

    /// Tries to fetch the user with the given id.
    pub fn fetch_team_id(&self, id: i32) -> Result<Team, Error> {
        use crate::schema::teams::dsl;
        let query = dsl::teams.filter(dsl::id.eq(id));
        match self {
            #[cfg(feature = "mysql")]
            DbConn::Mysql(conn) => query.first(conn),
            #[cfg(feature = "postgres")]
            DbConn::Postgres(conn) => query.first(conn),
            #[cfg(feature = "sqlite")]
            DbConn::Sqlite(conn) => query.first(conn),
        }
        .map_err(Error::from)
    }
}
