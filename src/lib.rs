//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

extern crate actix;
extern crate actix_web;
extern crate cfg_if;
extern crate either;
extern crate failure;
extern crate regex;
#[macro_use]
extern crate tera;
extern crate walkdir;

pub mod app;
pub mod challenge;
pub mod config;
pub mod views;

pub use app::{AppState, OpenCTF};
pub use challenge::Challenge;
pub use config::Config;
