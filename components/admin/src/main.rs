extern crate env_logger;
#[macro_use]
extern crate log;
extern crate core;
extern crate toml;
#[macro_use]
extern crate structopt;

mod cmd;
mod chal;

use structopt::StructOpt;

use cmd::AdminCommand;

fn main() -> Result<(), core::Error> {
    env_logger::init();
    let opt = AdminCommand::from_args();
    opt.run()
}
