use std::fs::{read_dir, File};
use std::io::Read;
use std::path::PathBuf;

use toml;

use errors::{CustomError, DirError};
use Error;

lazy_static! {
    static ref REQUIRED_FIELDS: [&'static str; 3] = ["title", "description_file", "grader_file"];
}

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

            // read the problem file
            let problem_contents = match {
                let mut file = match File::open(&problem_toml_path) {
                    Ok(file) => file,
                    Err(err) => {
                        return Err(CustomError::new(format!("Error accessing file: {}", err)).into())
                    }
                };
                let mut contents = String::new();
                match file.read_to_string(&mut contents) {
                    Ok(_) => Ok::<_, Error>(contents),
                    Err(err) => {
                        Err(CustomError::new(format!("Error reading file: {}", err)).into())
                    }
                }
            } {
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
            let mut flag = false;
            for field in REQUIRED_FIELDS.iter() {
                if !problem_toml.contains_key(field.to_owned()) {
                    failed.push((
                        Some(path),
                        CustomError::new(format!(
                            "problem.toml is missing required key '{}'",
                            field
                        )).into(),
                    ));
                    flag = true;
                    break;
                }
            }
            if flag {
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
