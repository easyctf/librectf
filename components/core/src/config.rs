use std::fs::File;
use std::path::Path;

use cfg::{self, Environment};
use failure::Error;
use serde::Deserialize;

pub trait Config<'d>: Sized + Deserialize<'d> {
    fn new<P>(file: Option<P>) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let mut c = cfg::Config::new();

        // optionally load from file if provided
        if let Some(path) = file {
            c.merge::<cfg::File<_>>(path.as_ref().to_path_buf().into())?;
        }

        c.merge(Environment::with_prefix("librectf"))?;
        c.try_into().map_err(|err| err.into())
    }
}

impl<'d, T: Deserialize<'d>> Config<'d> for T {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedisConfig {
    uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommonConfig {
    redis: RedisConfig,
}
