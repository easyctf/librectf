use std::ops::Deref;

use core::{self, config::AdminConfig};

pub struct Config {
    pub database_url: String,
    pub inner: Option<AdminConfig>,
}

impl Deref for Config {
    type Target = Option<AdminConfig>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<core::Config> for Config {
    fn from(config: core::Config) -> Self {
        Config {
            database_url: config.database_url.clone(),
            inner: config.admin.clone(),
        }
    }
}
