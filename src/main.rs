extern crate actix_web;
extern crate config as cfg;
extern crate env_logger;
#[macro_use]
extern crate failure;
extern crate serde;
#[macro_use]
extern crate structopt;
extern crate serde_derive;

extern crate admin;
// extern crate api;
extern crate core;
extern crate filestore;
extern crate frontend;

mod web;

use std::path::PathBuf;

use core::config::{Config, ReadConfig};
use failure::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    /// Admin utilities.
    #[structopt(name = "admin")]
    Admin(admin::AdminCommand),

    /// Runs a web server.
    #[structopt(name = "web")]
    Web(web::WebCommand),
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// Configuration file to use.
    #[structopt(long = "config-file", parse(from_os_str))]
    config_file: Option<PathBuf>,

    #[structopt(subcommand)]
    command: Command,
}

fn run() -> Result<(), Error> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn");
    env_logger::Builder::from_env(env).init();

    let config = Config::new(None)?;
    let args = Opt::from_args();

    match args.command {
        Command::Admin(cmd) => cmd.run(&config),
        Command::Web(web) => web.run(&config),
    }
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => eprintln!("Error: {}\n{}", err, err.backtrace()),
    }
}
