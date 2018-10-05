extern crate failure;
extern crate openctf;
extern crate structopt;

use openctf::cli::OpenCTF;
use openctf::Error;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opt = OpenCTF::from_args();
    opt.run()
}
