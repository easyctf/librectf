use std::fs::read_dir;
use std::path::PathBuf;

use toml;

use errors::{CustomError, DirTraversalError};
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
        for entry in read_dir(&self.challenge_dir).map_err(|err| DirTraversalError(err))? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    failed.push((None, DirTraversalError(err).into()));
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

            // TODO: run Make here

            // find meta.toml
            let meta_toml_path = path.join("meta.toml");
            if !meta_toml_path.exists() {
                failed.push((
                    Some(path),
                    CustomError::new("Could not find meta.toml in this directory.").into(),
                ));
                continue;
            }

            // read the meta file
            let meta_contents = match read_file(&meta_toml_path) {
                Ok(meta) => meta,
                Err(err) => {
                    failed.push((Some(path), err));
                    continue;
                }
            };

            // parse the meta file
            let meta_toml = match toml::from_str::<toml::Value>(&meta_contents) {
                Ok(value) => value,
                Err(err) => {
                    failed.push((
                        Some(path),
                        CustomError::new(format!("Deserialization error: {}", err)).into(),
                    ));
                    continue;
                }
            };
            let meta_toml = match meta_toml {
                toml::Value::Table(table) => table,
                _ => {
                    failed.push((
                        Some(path),
                        CustomError::new("meta.toml must be a TOML table.").into(),
                    ));
                    continue;
                }
            };

            // check for the existence of required fields
            let title = match meta_toml.get("title") {
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
                meta_toml.get("description"),
                meta_toml.get("description_file"),
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
            let grader = match meta_toml.get("grader_file") {
                Some(toml::Value::String(grader_path)) => {
                    let path = path.join(&grader_path);
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
                        CustomError::new("String key 'grader_file' not found.").into(),
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
