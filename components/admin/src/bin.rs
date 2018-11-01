extern crate admin;
extern crate env_logger;
extern crate failure;
extern crate structopt;

use admin::AdminCommand;
use failure::Error;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    env_logger::init();

    let opt = AdminCommand::from_args();
    opt.run()
}
