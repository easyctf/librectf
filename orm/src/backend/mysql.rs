pub(crate) extern crate r2d2_mysql;

use std::sync::Arc;

use self::r2d2_mysql::MysqlConnectionManager;
use r2d2::Pool;

use AsQuery;
use Backend;
use ConnectionPoolExt;
use QueryBuilder;

pub struct MysqlBackend {}

impl Backend for MysqlBackend {
    type QueryBuilder = MysqlQueryBuilder;
    type ConnectionManager = MysqlConnectionManager;
}

pub struct MysqlConnectionPool {
    pool: Arc<Pool<MysqlConnectionManager>>,
}

impl MysqlConnectionPool {
    pub fn new(database_url: impl AsRef<str>) -> Self {
        use backend::mysql::r2d2_mysql::{
            mysql::{Opts, OptsBuilder},
            MysqlConnectionManager,
        };
        use r2d2::Pool;

        let opts = Opts::from_url(database_url.as_ref()).unwrap();
        let builder = OptsBuilder::from_opts(opts);
        let manager = MysqlConnectionManager::new(builder);

        // TODO: don't hardcode max pool size
        let pool = Arc::new(Pool::builder().max_size(4).build(manager).unwrap());
        MysqlConnectionPool { pool }
    }
}

impl ConnectionPoolExt for MysqlConnectionPool {
    fn run(&self, query: impl AsQuery) {}
}

#[derive(Default)]
pub struct MysqlQueryBuilder {}

impl QueryBuilder<MysqlBackend> for MysqlQueryBuilder {
    fn build(&self) -> String {
        // TODO:
        String::new()
    }
}
