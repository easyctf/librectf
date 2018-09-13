//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

#![deny(missing_docs)]

extern crate rocket;
extern crate failure;

mod challenge;

pub use challenge::Challenge;

