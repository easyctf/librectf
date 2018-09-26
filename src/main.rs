extern crate failure;
extern crate openctf;
extern crate structopt;

use openctf::cli::OpenCTF;
use structopt::StructOpt;

fn main() {
    let opt = OpenCTF::from_args();
    match opt.run() {
        Ok(_) => (),
        Err(err) => panic!("Error occurred: {}", err),
    }
}
