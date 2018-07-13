//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

extern crate actix;
extern crate actix_web;
#[macro_use]
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
pub mod templates;

pub use app::OpenCTF;
pub use challenge::Challenge;
pub use config::Config;
pub use templates::Templates;
