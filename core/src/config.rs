use std::net::SocketAddr;

use crate::db::DatabaseUri;

/// Config.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// The address for the web server to bind to.
    bind_addr: SocketAddr,

    /// Uri to reach the database. See the `DatabaseUri` documentation for the
    /// format of the connection string.
    database_uri: DatabaseUri,
}
