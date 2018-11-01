extern crate config as cfg;
extern crate env_logger;
extern crate serde;
extern crate structopt;
#[macro_use]
extern crate serde_derive;

extern crate api;
extern crate core;

mod config;

use std::path::PathBuf;

use structopt::StructOpt;

use config::Config;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    config_file: Option<PathBuf>,
}

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn");
    env_logger::Builder::from_env(env).init();

    let args = Opt::from_args();
    println!("{:?}", args);
}
