extern crate core;
extern crate diesel;
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
mod config;
mod util;

use failure::Error;
use structopt::StructOpt;

use chal::ImportChalCommand;
use config::Config;
pub use config::ConfigWrapper;

#[derive(StructOpt)]
pub enum AdminCommand {
    /// Import challenges from a directory.
    #[structopt(name = "import")]
    Import(ImportChalCommand),
}

pub fn run(cmd: &AdminCommand, config: &Config) -> Result<(), Error> {
    match cmd {
        AdminCommand::Import(cmd) => cmd.run(config),
    }
}
