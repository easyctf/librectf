use std::sync::Arc;

use crate::{DbConn, DbPool, Error};

#[derive(Clone)]
pub struct State {
    secret_key: Arc<String>,
    db: Arc<DbPool>,
}

impl State {
    pub fn new(secret_key: impl AsRef<str>, db: Arc<DbPool>) -> Self {
        let secret_key = Arc::new(secret_key.as_ref().to_owned());
        State { secret_key, db }
    }

    pub fn get_secret_key(&self) -> Arc<String> {
        self.secret_key.clone()
    }

    pub fn get_connection(&self) -> Result<DbConn, Error> {
        self.db.get()
    }
}
