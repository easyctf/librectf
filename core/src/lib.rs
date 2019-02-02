#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

mod db;
mod errors;
mod models;
mod schema;

pub mod users;

pub use crate::db::{DatabaseUri, DbPool};
pub use crate::errors::{Error, UserError};
