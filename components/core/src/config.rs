use std::path::PathBuf;

use cfg::{self, Environment};
use serde::Deserialize;

use crate::Error;

pub trait ReadConfig<'d>: Sized + Deserialize<'d> {
    fn new(file: Option<PathBuf>) -> Result<Self, Error> {
        let mut c = cfg::Config::new();

        // optionally load from file if provided
        match file {
            Some(path) => {
                c.merge::<cfg::File<_>>(path.into())?;
            }
            None => {
                // hardcode in the filename 'librectf.toml'
                let path = PathBuf::from("librectf.toml");
                if path.exists() {
                    c.merge::<cfg::File<_>>(path.into())?;
                }
            }
        }

        c.merge(Environment::with_prefix("librectf"))?;
        c.try_into().map_err(|err| err.into())
    }
}

impl<'d, T: Deserialize<'d>> ReadConfig<'d> for T {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RedisConfig {
    pub uri: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MailCredentials {
    SMTP { host: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminConfig {
    pub filestore_url: String,
    pub filestore_push_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiConfig {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FilestoreConfig {
    pub push_password: String,
    pub pull_password: String,
    pub url_prefix: String,
    pub storage_dir: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebConfig {
    pub bind_host: String,
    pub bind_port: u16,
    pub secret_key: String,

    pub api: Option<ApiConfig>,
    pub filestore: Option<FilestoreConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub database_url: String,

    pub admin: Option<AdminConfig>,
    pub web: Option<WebConfig>,
}
