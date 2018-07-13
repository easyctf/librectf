//! # OpenCTF CLI

extern crate cli;
extern crate structopt;

use cli::OpenCTF;
use structopt::StructOpt;

fn main() {
    let opt = OpenCTF::from_args();
    match opt.run() {
        Ok(_) => (),
        Err(err) => panic!(err),
    }
}
