use core::Error;
use chal::ImportChalCommand;

#[derive(Debug, StructOpt)]
pub enum AdminCommand {
    /// Import challenges from a directory into the database
    #[structopt(name = "import")]
    Import(ImportChalCommand),
}

impl AdminCommand {
    pub fn run(&self) -> Result<(), Error> {
        match self {
            AdminCommand::Import(cmd) => cmd.run(),
        }
    }
}
