extern crate env_logger;
extern crate failure;
extern crate filestore;
extern crate structopt;

use failure::Error;
use filestore::FilestoreCommand;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    env_logger::init();

    let opt = FilestoreCommand::from_args();
    opt.run()
}
