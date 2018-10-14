extern crate failure;
extern crate openctf_core;
extern crate structopt;

use openctf_core::cli::OpenCTF;
use openctf_core::Error;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = OpenCTF::from_args();
    opt.run()
}
