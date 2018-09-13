//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

#![deny(missing_docs)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

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
pub mod web;

pub use challenge::Challenge;
