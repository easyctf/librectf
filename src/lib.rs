//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

#![feature(plugin, custom_derive, try_from)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

extern crate bcrypt;
#[macro_use]
extern crate embed;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate mime_guess;
extern crate regex;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
extern crate tera;

mod challenge;
mod config;
mod db;
pub mod models;
mod schema;
pub mod web;

pub use challenge::Challenge;
pub use config::Config;
