use std::path::PathBuf;

use failure::Error;

/// Represents a configuration for an OpenCTF server instance.
pub struct Config {
    /// The host on which to listen (default: '0.0.0.0')
    pub host: String,
    /// The port on which to listen (default: 4401)
    pub port: u16,
}

impl Config {
    /// Generates the default configuration for an OpenCTF server instance. This will also read
    /// from environment variables as necessary.
    pub fn default() -> Config {
        // TODO: use environment variables
        Config {
            host: "0.0.0.0".to_owned(),
            port: 4401,
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<Config, Error> {
        // TODO: parse a config from file
        Ok(Config::default())
    }
}
