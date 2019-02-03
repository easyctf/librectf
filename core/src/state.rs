use std::sync::Arc;

use crate::{DbPool};

#[derive(Clone)]
pub struct State {
    db: Arc<DbPool>,
}

impl State {
    pub fn new(db: Arc<DbPool>) -> Self {
        State { db }
    }
}
