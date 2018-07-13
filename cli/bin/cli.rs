//! # OpenCTF CLI

extern crate cli;

use cli::OpenCTF;

fn main() {
    match OpenCTF::run() {
        Ok(_) => (),
        Err(err) => panic!(err),
    }
}
