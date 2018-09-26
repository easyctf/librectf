mod chal;
mod web;

use std::path::PathBuf;

use failure::Error;
use structopt::StructOpt;

use self::web::Web;

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
