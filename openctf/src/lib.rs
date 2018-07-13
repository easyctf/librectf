//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

#[macro_use]
extern crate actix;
extern crate actix_web;
extern crate failure;

pub mod app;
pub mod challenge;
pub mod config;

pub use app::OpenCTF;
pub use challenge::Challenge;
pub use config::Config;
