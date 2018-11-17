extern crate api;
extern crate core;
extern crate env_logger;
extern crate failure;
extern crate structopt;

use std::path::PathBuf;

use core::Config;
use failure::Error;
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
    let config = api::ConfigWrapper::new(opt.config_file)?;
    api::run(config.api)
}
