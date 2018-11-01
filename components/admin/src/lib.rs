extern crate core;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
extern crate serde;
extern crate toml;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

mod chal;
mod cmd;

use structopt::StructOpt;

pub use cmd::AdminCommand;
