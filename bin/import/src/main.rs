extern crate env_logger;
#[macro_use]
extern crate log;
extern crate openctf_core;
extern crate toml;
#[macro_use]
extern crate structopt;

mod cmd;

use structopt::StructOpt;

use cmd::ImportCommand;

fn main() -> Result<(), openctf_core::Error> {
    env_logger::init();
    let opt = ImportCommand::from_args();
    opt.run()
}
