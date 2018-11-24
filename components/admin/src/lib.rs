extern crate core;
extern crate diesel;
#[macro_use]
extern crate embed;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate migrations_internals;
extern crate multipart;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;
extern crate tempfile;
extern crate toml;

mod chal;
mod config;
mod migrate;
mod util;

use failure::Error;
use structopt::StructOpt;

use chal::ImportChalCommand;
use config::Config;
pub use config::ConfigWrapper;
use migrate::MigrateCommand;

#[derive(Embed)]
#[folder = "migrations"]
struct Migrations;

#[derive(StructOpt)]
pub enum AdminCommand {
    /// Import challenges from a directory.
    #[structopt(name = "import")]
    Import(ImportChalCommand),

    #[structopt(name = "migrate")]
    Migrate(MigrateCommand),
}

pub fn run(cmd: &AdminCommand, config: &Config) -> Result<(), Error> {
    match cmd {
        AdminCommand::Import(cmd) => cmd.run(config),
        AdminCommand::Migrate(cmd) => cmd.run(config),
    }
}
