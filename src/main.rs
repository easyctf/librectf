extern crate failure;
extern crate openctf;
extern crate structopt;

use std::path::PathBuf;

use failure::Error;
use openctf::Config;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Web {
    #[structopt(flatten)]
    config: Config,
}

impl Web {
    pub fn run(&self) {
        let app = openctf::web::app(&self.config);
        app.launch();
    }
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Run a web server.
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
        match &self.cmd {
            Command::Web(web) => web.run(),
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
