use std::sync::Arc;

use crate::{DbConn, DbPool, Error};

/// The state associated with the running application.
///
/// Since everything inside this is a reference-counted pointer, this struct
/// implements `Clone` to allow it to be easily passed around.
#[derive(Clone)]
pub struct State {
    secret_key: Arc<String>,
    db: Arc<DbPool>,
}

impl State {
    /// Create a new state with the given defaults.
    pub fn new(secret_key: impl AsRef<str>, db: Arc<DbPool>) -> Self {
        let secret_key = Arc::new(secret_key.as_ref().to_owned());
        State { secret_key, db }
    }

    /// Get a reference to the secret key.
    pub fn get_secret_key(&self) -> Arc<String> {
        self.secret_key.clone()
    }

    /// Get a connection from the connection pool.
    pub fn get_connection(&self) -> Result<DbConn, Error> {
        self.db.get()
    }
}
