//! # LibreCTF
//!
//! This crate contains the core of the LibreCTF library.

#![allow(proc_macro_derive_resolution_fallback)]

extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
extern crate comrak;
extern crate config as cfg;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
extern crate futures;
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate redis_async;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate structopt;
extern crate tera;

#[macro_use]
mod macros;

pub mod chal;
pub mod config;
pub mod db;
pub mod models;
pub mod pages;
pub mod schema;
mod state;
pub mod user;

pub use config::Config;
pub use db::{establish_connection, Pool};
pub use state::State;
