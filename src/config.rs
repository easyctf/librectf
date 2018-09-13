use std::path::PathBuf;

use failure::Error;

/// Represents a configuration for an OpenCTF server instance.
pub struct Config {
}

impl Config {
    pub fn default() -> Config {
        Config {}
    }

    pub fn from_file(path: &PathBuf) -> Result<Config, Error> {
        // TODO: parse a config from file
        Ok(Config::default())
    }
}
