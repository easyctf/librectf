//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

// #![feature(custom_derive, tool_lints, try_from)]
// #![allow(clippy::needless_pass_by_value)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate actix_web;
extern crate base64;
extern crate bcrypt;
extern crate cache;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate idna;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mime_guess;
extern crate regex;
extern crate serde;
extern crate serde_cbor;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
extern crate task_queue;
extern crate toml;

mod challenge;
pub mod cli;
mod config;
mod db;
mod errors;
pub mod models;
mod schema;
mod tasks;
mod util;
pub mod web;

pub use challenge::Challenge;
pub use config::Config;
pub use errors::Error;

const INTERNAL_SERVER_ERROR_MESSAGE: &str = "Internal server error, please contact the webmaster.";
