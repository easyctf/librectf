//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

#![feature(plugin, custom_attribute, proc_macro_mod)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate embed;
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate mime_guess;
#[macro_use]
extern crate orm;
extern crate orm_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate structopt;
extern crate tera;

mod challenge;
mod config;
pub mod models;
pub mod web;

pub use challenge::Challenge;
pub use config::Config;
