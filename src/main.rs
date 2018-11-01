extern crate actix_web;
extern crate config as cfg;
extern crate env_logger;
extern crate failure;
extern crate serde;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;

extern crate admin;
extern crate api;
extern crate core;
extern crate filestore;

mod config;
mod web;

use std::path::PathBuf;

use structopt::StructOpt;

use config::Config;

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

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn");
    env_logger::Builder::from_env(env).init();

    let args = Opt::from_args();
    println!("{:?}", args);
}
