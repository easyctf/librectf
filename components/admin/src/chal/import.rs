use std::fs::{read_dir, read_to_string, DirEntry};
use std::path::PathBuf;

use failure::Error;
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct Autogen {
    generator: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    #[serde(skip, default)]
    name: String,
    title: String,
    value: i32,
    description: String,
    grader: String,
    autogen: Option<Autogen>,
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
            if let Err(err) = (|entry| -> Result<(), Error> {
                let entry: DirEntry = entry?;
                let path = entry.path();
                // TODO unfuck this
                let name = path.file_name().unwrap().to_str().unwrap().to_owned();

                // skip names that begin with '.'
                if let Some(true) = path
                    .file_name()
                    .and_then(|ostr| ostr.to_str())
                    .map(|name| !name.starts_with("."))
                {
                } else {
                    return Ok(());
                }

                // TODO: run Make here

                // find meta.toml
                let meta_toml_path = path.join("meta.toml");
                if !meta_toml_path.exists() {
                    bail!("Could not find meta.toml in this directory.");
                }

                // read and the meta file
                let meta_contents = read_to_string(&meta_toml_path)?;
                let meta_toml = toml::from_str::<Metadata>(&meta_contents).map(|mut value| {
                    value.name = name;
                    value
                })?;

                println!("Successfully loaded: {:?}", meta_toml);
                Ok(())
            })(entry)
            {
                failed.push((None, format_err!("Error loading: {}", err)));
            }
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
