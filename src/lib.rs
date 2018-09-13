//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

#![deny(missing_docs)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate config as _config;
extern crate diesel;
#[macro_use]
extern crate embed;
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate mime_guess;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate tera;

mod challenge;
mod config;
mod db;
pub mod web;

pub use challenge::Challenge;
pub use config::Config;
