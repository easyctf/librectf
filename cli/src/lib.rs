extern crate failure;
#[macro_use]
extern crate structopt;

mod web;

use failure::Error;
use structopt::StructOpt;

use web::Web;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "web")]
    Web(Web),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "openctf", about = "Flexible and powerful CTF framework.")]
pub struct OpenCTF {
    #[structopt(subcommand)]
    cmd: Command,
}

impl OpenCTF {
    pub fn run() -> Result<(), Error> {
        let opt = OpenCTF::from_args();
        println!("{:?}", opt);
        Ok(())
    }
}
