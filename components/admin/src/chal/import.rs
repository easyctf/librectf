use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, DirEntry};
use std::path::PathBuf;

use core::models::{Challenge, NewChallenge, NewFile};
use diesel::{prelude::*, result::Error::RollbackTransaction};
use failure::Error;
use hyper::{client::Request, header::Authorization, method::Method};
use multipart::client::Multipart;
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
    files: Option<HashMap<String, String>>,
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

// TODO: update instead of insert if it already exists
impl ImportChalCommand {
    pub fn run(&self, config: &Config) -> Result<(), Error> {
        let mut files_to_add = Vec::<NewFile>::new();
        let mut chals_to_add = Vec::<(i32, NewChallenge)>::new();

        let mut failed = Vec::<(Option<PathBuf>, Error)>::new();
        let mut files = Vec::<(i32, String, PathBuf)>::new();

        // hack to get the files to have challenge ids
        let mut counter = 0;
        let mut chal_id_map = HashMap::<i32, i32>::new();

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

                // TODO: build files in the challenge here

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

                // TODO: queue up uploading files into filestore
                if let Some(meta_files) = &meta.files {
                    for (name, file) in meta_files {
                        let file_path = path.join(file);
                        if !file_path.exists() {
                            continue;
                        }

                        files.push((counter, name.to_owned(), file_path));
                    }
                }

                // add it to the database
                let chal = meta.into();
                chals_to_add.push((counter, chal));
                counter += 1;
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

        // add files into filestore
        if files.len() > 0 {
            let filestore_url = match &config.filestore_url {
                Some(string) => string,
                None => bail!("Please include a filestore URL."),
            };
            let filestore_push_password = match &config.filestore_push_password {
                Some(string) => string,
                None => bail!("Please include a filestore push password."),
            };

            for (id, name, path) in files {
                let mut request = Request::new(Method::Post, filestore_url.parse()?)?;
                {
                    let mut headers = request.headers_mut();
                    headers.set(Authorization(filestore_push_password.clone()));
                }

                let mut multipart = Multipart::from_request(request)?;
                multipart.write_file("file", &path)?;
                let response = multipart.send()?;
                info!("Filestore response: {:?}", response);

                let new_file = NewFile {
                    name,
                    url: String::from("lol"),
                    chal_id: id,
                    team_id: None,
                };
                files_to_add.push(new_file);
            }
        }

        let conn = establish_connection(config)
            .expect("Couldn't connect to database. Did you specify DATABASE_URL?");

        {
            use core::schema::{chals::dsl::chals, files::dsl::files};

            for (id, item) in chals_to_add {
                conn.transaction(|| {
                    if let Err(err) = diesel::insert_into(chals).values(item).execute(&conn) {
                        error!("get fucked {}", err);
                        return Err(RollbackTransaction);
                    };

                    let new_chal = match {
                        use core::schema::chals::dsl::id;
                        chals.order_by(id.desc()).first::<Challenge>(&conn)
                    } {
                        Ok(team) => team,
                        Err(err) => {
                            error!("get fucked {}", err);
                            return Err(RollbackTransaction);
                        }
                    };

                    chal_id_map.insert(id, new_chal.id);
                    Ok(())
                })?;
            }

            for mut item in files_to_add.iter_mut() {
                item.chal_id = *chal_id_map.get(&item.chal_id).unwrap();
            }
            diesel::insert_into(files)
                .values(&files_to_add)
                .execute(&conn)?;

            info!("Successfully imported challenge(s).");
            Ok(())
        }
    }
}
