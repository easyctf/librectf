#[macro_use]
extern crate log;
extern crate core;
extern crate toml;
#[macro_use]
extern crate structopt;

mod chal;
mod cmd;

use structopt::StructOpt;

pub use cmd::AdminCommand;
