extern crate admin;
extern crate core;
extern crate env_logger;
extern crate failure;
extern crate structopt;

use core::Config;
use std::path::PathBuf;

use admin::AdminCommand;
use failure::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(flatten)]
    command: AdminCommand,

    #[structopt(long = "config-file", parse(from_os_str))]
    config_file: Option<PathBuf>,
}

fn main() -> Result<(), Error> {
    env_logger::init();

    let opt = Opt::from_args();

    let config = admin::ConfigWrapper::new(opt.config_file)?;
    admin::run(&opt.command, &config.admin)
}
