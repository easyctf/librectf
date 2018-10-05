use std::fs::read_dir;
use std::path::PathBuf;

use errors::{CustomError, DirError};
use Error;

#[derive(Debug, StructOpt)]
pub enum ChalCommand {
    /// Import challenges from a directory into the database
    #[structopt(name = "import")]
    Import(ChalImportCommand),
}

impl ChalCommand {
    pub fn run(&self) -> Result<(), Error> {
        match self {
            ChalCommand::Import(import) => import.run(),
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct ChalImportCommand {
    /// Root challenge directory
    #[structopt(parse(from_os_str))]
    challenge_dir: PathBuf,
}

impl ChalImportCommand {
    pub fn run(&self) -> Result<(), Error> {
        let mut failed: Vec<(Option<PathBuf>, Error)> = Vec::new();
        for entry in read_dir(&self.challenge_dir).map_err(|err| DirError(err))? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    failed.push((None, DirError(err).into()));
                    continue;
                }
            };
            let path = entry.path();

            // find problem.toml
            let problem_toml_path = path.join("problem.toml");
            if !problem_toml_path.exists() {
                failed.push((
                    Some(path),
                    CustomError::new("Could not find problem.toml in this directory.").into(),
                ));
                continue;
            }
        }

        if failed.len() > 0 {
            error!("Failed to load directories:");
            for (path, err) in failed {
                error!(" - {:?}: {}", path, err);
            }
        }
        Ok(())
    }
}
