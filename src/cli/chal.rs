use std::fs::{read_dir, File};
use std::io::Read;
use std::path::PathBuf;

use toml;

use errors::{CustomError, DirError};
use util::read_file;
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

            match path
                .file_name()
                .and_then(|ostr| ostr.to_str())
                .and_then(|name| {
                    if name.starts_with(".") {
                        None
                    } else {
                        Some(())
                    }
                }) {
                Some(_) => (),
                None => continue,
            }

            // find problem.toml
            let problem_toml_path = path.join("problem.toml");
            if !problem_toml_path.exists() {
                failed.push((
                    Some(path),
                    CustomError::new("Could not find problem.toml in this directory.").into(),
                ));
                continue;
            }

            // read the problem file
            let problem_contents = match read_file(&problem_toml_path) {
                Ok(problem) => problem,
                Err(err) => {
                    failed.push((Some(path), err));
                    continue;
                }
            };

            // parse the problem file
            let problem_toml = match toml::from_str::<toml::Value>(&problem_contents) {
                Ok(value) => value,
                Err(err) => {
                    failed.push((
                        Some(path),
                        CustomError::new(format!("Deserialization error: {}", err)).into(),
                    ));
                    continue;
                }
            };
            let problem_toml = match problem_toml {
                toml::Value::Table(table) => table,
                _ => {
                    failed.push((
                        Some(path),
                        CustomError::new("problem.toml must be a TOML table.").into(),
                    ));
                    continue;
                }
            };

            // check for the existence of required fields
            let title = match problem_toml.get("title") {
                Some(toml::Value::String(title)) => title,
                _ => {
                    failed.push((
                        Some(path),
                        CustomError::new("String key 'title' not found.").into(),
                    ));
                    continue;
                }
            };
            let description = match (
                problem_toml.get("description"),
                problem_toml.get("description_file"),
            ) {
                (Some(_), Some(_)) => {
                    failed.push((
                        Some(path),
                        CustomError::new("Cannot use both 'description' and 'description_file'.")
                            .into(),
                    ));
                    continue;
                }
                (Some(toml::Value::String(description)), None) => description.clone(),
                (None, Some(toml::Value::String(description_file))) => {
                    match read_file(&description_file) {
                        Ok(description) => description,
                        Err(err) => {
                            failed.push((Some(path), err.into()));
                            continue;
                        }
                    }
                }
                _ => {
                    failed.push((
                        Some(path),
                        CustomError::new("Neither 'description' nor 'description_file' found.")
                            .into(),
                    ));
                    continue;
                }
            };
            let grader = match problem_toml.get("grader") {
                Some(toml::Value::String(grader_path)) => {
                    let path = PathBuf::from(&grader_path);
                    if !path.exists() {
                        failed.push((
                            Some(path),
                            CustomError::new("Grader file not found.").into(),
                        ));
                        continue;
                    }
                    path
                }
                _ => {
                    failed.push((
                        Some(path),
                        CustomError::new("String key 'grader' not found.").into(),
                    ));
                    continue;
                }
            };
        }

        if failed.len() > 0 {
            error!("Failed to load directories:");
            for (path, err) in failed {
                error!(" - {:?}: {:?}", path, err);
            }
            return Err(CustomError::new("Failed to import some challenges.").into());
        }
        info!("Successfully imported all challenges.");
        Ok(())
    }
}
