extern crate core;
extern crate env_logger;
extern crate failure;
extern crate filestore;
extern crate structopt;

use std::path::PathBuf;

use core::Config;
use failure::Error;
use filestore::FilestoreCommand;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long = "config-file", parse(from_os_str))]
    config_file: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    env_logger::init();

    // parse cmd line arguments
    let opt = Opt::from_args();

    // read configuration
    let cmd = FilestoreCommand::new(opt.config_file)?;
    cmd.run()
}
