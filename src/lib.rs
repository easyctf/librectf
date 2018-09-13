//! # OpenCTF
//!
//! This crate contains the core of the OpenCTF library.

#![deny(missing_docs)]

extern crate failure;

mod challenge;
mod web;

pub use challenge::Challenge;
