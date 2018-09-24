//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

#[macro_use]
extern crate embed;
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate mime_guess;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate structopt;
extern crate tera;

mod challenge;
mod config;
mod db;
pub mod models;
pub mod web;

pub use challenge::Challenge;
pub use config::Config;
