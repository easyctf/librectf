use std::net::SocketAddr;

use crate::db::DatabaseUri;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    bind_addr: SocketAddr,
    database_uri: DatabaseUri,
}
