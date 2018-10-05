mod chal;
mod web;

use std::path::PathBuf;

use env_logger;
use structopt::StructOpt;

use self::chal::ChalCommand;
use self::web::WebCommand;
use Error;

#[derive(Debug, StructOpt)]
enum Command {
    /// Challenge-related commands
    #[structopt(name = "chal")]
    Chal(ChalCommand),

    /// Run a web server.
    #[structopt(name = "web")]
    Web(WebCommand),
}

impl Command {
    pub fn run(&self) -> Result<(), Error> {
        match self {
            Command::Chal(chal) => chal.run(),
            Command::Web(web) => web.run(),
        }
    }
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
        env_logger::init();
        self.cmd.run()
    }
}
