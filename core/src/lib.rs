//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

// #![feature(custom_derive, tool_lints, try_from)]
// #![allow(clippy::needless_pass_by_value)]
#![allow(proc_macro_derive_resolution_fallback)]
// #![deny(missing_docs)]


extern crate cache;
extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate lazy_static;
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
extern crate serde;
extern crate serde_json;
extern crate structopt;
extern crate task_queue;
extern crate toml;

#[macro_use]
mod macros;

pub mod errors;
pub mod models;
pub mod schema;

mod db;
mod tasks;

pub use errors::Error;
pub use db::{establish_connection, Pool};
