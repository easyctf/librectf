#![deny(missing_docs)]

//! This crate contains the "core" data structures and functions used by the
//! other components of the LibreCTF platform.
//!
//! - Database [**models**][models] include [Challenge][models::Challenge],
//!   [Invitation][models::Invitation], [File][models::File], [User][models::User],
//!   [Team][models::Team].

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;

mod config;
mod db;
mod errors;
#[doc(flatten)]
pub mod models;
mod schema;
mod state;

#[doc(flatten)]
pub mod users;

pub use crate::config::Config;
pub use crate::db::{DatabaseUri, DbConn, DbPool};
pub use crate::errors::{Error, ErrorKind, UserErrorKind};
pub use crate::state::State;
