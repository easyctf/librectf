extern crate env_logger;
extern crate failure;
extern crate openctf;
extern crate structopt;

mod web;

use std::path::PathBuf;

use failure::{err_msg, Error};
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
    pub fn run(&self) -> Result<(), Error> {
        let config = openctf::Config::new().map_err(|err| err_msg(err))?;
        match &self.cmd {
            Command::Web(web) => web.run(&config),
        }
        Ok(())
    }
}

fn main() {
    let opt = OpenCTF::from_args();
    match opt.run() {
        Ok(_) => (),
        Err(err) => panic!("Error occurred: {}", err),
    }
}
