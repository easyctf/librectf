extern crate env_logger;
extern crate failure;
extern crate openctf;
#[macro_use]
extern crate structopt;

mod web;

use std::path::PathBuf;

use failure::Error;
use openctf::Config;
use structopt::StructOpt;

use web::Web;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "web")]
    Web(Web),
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "openctf",
    about = "Flexible and powerful CTF framework."
)]
pub struct OpenCTF {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(
        short = "c",
        long = "config",
        help = "Path to the config file.",
        parse(from_os_str)
    )]
    config_file: Option<PathBuf>,
}

impl OpenCTF {
    pub fn run(self) -> Result<(), Error> {
        match &self.cmd {
            &Command::Web(ref web) => web.run(self.get_config()?),
        }
    }

    fn get_config(&self) -> Result<Config, Error> {
        match &self.config_file {
            Some(ref path) => Config::from_file(path),
            None => Ok(Config::default()),
        }
    }
}

fn main() {
    let opt = OpenCTF::from_args();
    match opt.run() {
        Ok(_) => (),
        Err(err) => panic!("Error occurred: {}", err),
    }
}
