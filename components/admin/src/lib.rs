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

use config::Config;
use failure::Error;
use structopt::StructOpt;

use chal::ImportChalCommand;
use migrate::MigrateCommand;

#[derive(Embed)]
#[folder = "migrations"]
struct Migrations;

#[derive(Debug, StructOpt)]
pub enum AdminCommand {
    /// Import challenges from a directory.
    #[structopt(name = "import")]
    Import(ImportChalCommand),

    #[structopt(name = "migrate")]
    Migrate(MigrateCommand),
}

impl AdminCommand {
    pub fn run(&self, config: &core::Config) -> Result<(), Error> {
        let config = config.clone();
        match self {
            AdminCommand::Import(cmd) => cmd.run(&config.into()),
            AdminCommand::Migrate(cmd) => cmd.run(&config.into()),
        }
    }
}
