//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

// #![feature(custom_derive, tool_lints, try_from)]
// #![allow(clippy::needless_pass_by_value)]
#![allow(proc_macro_derive_resolution_fallback)]
// #![deny(missing_docs)]

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate actix_web;
extern crate base64;
extern crate bcrypt;
extern crate cache;
extern crate chrono;
extern crate cookie;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate idna;
extern crate jsonwebtoken;
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mime_guess;
extern crate regex;
extern crate serde;
extern crate serde_cbor;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
extern crate task_queue;
extern crate toml;

#[macro_use]
mod macros;

pub mod cli;
mod db;
mod errors;
pub mod models;
mod schema;
mod tasks;
mod util;
pub mod web;

pub use errors::Error;