use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;

use failure::Error;
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct Autogen {
    generator: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    title: String,
    value: i32,
    description: String,
    grader: String,
    autogen: Autogen,
}

#[derive(Debug, StructOpt)]
pub struct ImportChalCommand {
    /// Root challenge directory
    #[structopt(parse(from_os_str))]
    challenge_dir: PathBuf,
}

impl ImportChalCommand {
    pub fn run(&self) -> Result<(), Error> {
        let mut failed: Vec<(Option<PathBuf>, Error)> = Vec::new();
        for entry in read_dir(&self.challenge_dir)? {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    failed.push((None, format_err!("Directory traversal error: {}", err)));
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
                    format_err!("Could not find meta.toml in this directory."),
                ));
                continue;
            }

            // read the meta file
            let meta_contents = match read_to_string(&meta_toml_path) {
                Ok(meta) => meta,
                Err(err) => {
                    failed.push((Some(path), format_err!("Failed to read file: {}", err)));
                    continue;
                }
            };

            // parse the meta file
            let meta_toml = match { toml::from_str::<Metadata>(&meta_contents) } {
                Ok(value) => value,
                Err(err) => {
                    failed.push((Some(path), format_err!("Deserialization error: {}", err)));
                    continue;
                }
            };

            println!("Successfully loaded: {:?}", meta_toml);
        }

        if failed.len() > 0 {
            error!("Failed to load directories:");
            for (path, err) in failed {
                error!(" - {:?}: {}", path, err);
            }
            return Err(format_err!("Failed to import some challenges."));
        }
        info!("Successfully imported all challenges.");
        Ok(())
    }
}
