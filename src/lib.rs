//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

extern crate actix;
extern crate actix_web;
extern crate cfg_if;
#[macro_use]
extern crate diesel;
extern crate either;
extern crate failure;
extern crate r2d2;
extern crate regex;
#[macro_use]
extern crate tera;
extern crate walkdir;
#[macro_use]
extern crate wtforms;

pub mod app;
pub mod challenge;
pub mod config;
pub(crate) mod db;
pub(crate) mod models;
pub mod views;

pub use app::{AppState, OpenCTF};
pub use challenge::Challenge;
pub use config::Config;
