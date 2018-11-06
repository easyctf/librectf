//! # LibreCTF
//!
//! This crate contains the core of the LibreCTF library.

// #![feature(custom_derive, tool_lints, try_from)]
// #![allow(clippy::needless_pass_by_value)]
#![allow(proc_macro_derive_resolution_fallback)]
// #![deny(missing_docs)]

extern crate chrono;
extern crate config as cfg;
#[macro_use]
extern crate diesel;
extern crate failure;
extern crate futures;
extern crate lazy_static;
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

#[macro_use]
mod macros;

pub mod api;
mod config;
pub mod db;
pub mod models;
pub mod schema;

mod tasks;

pub use config::Config;
pub use db::{establish_connection, Pool};
