use std::fs::{read_dir, read_to_string, DirEntry};
use std::path::PathBuf;

use core::models::NewChallenge;
use diesel::prelude::*;
use failure::Error;
use toml;

use config::Config;
use util::establish_connection;

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    #[serde(skip, default)]
    name: String,
    title: String,
    value: i32,
    description: String,
    regex: bool,
    flag: String,
}

impl Into<NewChallenge> for Metadata {
    fn into(self) -> NewChallenge {
        NewChallenge {
            title: self.title,
            enabled: true,
            value: self.value,
            description: self.description,
            regex: self.regex,
            correct_flag: self.flag,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct ImportChalCommand {
    /// Root challenge directory
    #[structopt(parse(from_os_str))]
    challenge_dir: PathBuf,
}

impl ImportChalCommand {
    pub fn run(&self, config: &Config) -> Result<(), Error> {
        let mut to_add = Vec::<NewChallenge>::new();
        let mut failed = Vec::<(Option<PathBuf>, Error)>::new();

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
                let meta = toml::from_str::<Metadata>(&meta_contents).map(|mut value| {
                    value.name = name;
                    value
                })?;

                println!("Successfully loaded: {:?}", meta);

                // TODO: save required files into filestore
                // TODO: add it to the database
                let chal = meta.into();
                to_add.push(chal);
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

        let conn = establish_connection(config)
            .expect("Couldn't connect to database. Did you specify DATABASE_URL?");

        {
            use core::schema::chals::dsl::chals;
            diesel::insert_into(chals)
                .values(&to_add)
                .execute(&conn)
                .map(|n| info!("Successfully imported {} challenge(s).", n))
                .map_err(|err| err.into())
        }
    }
}
