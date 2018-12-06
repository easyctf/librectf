use std::ops::Deref;

use core::{self, config::AdminConfig};

pub struct Config {
    pub database_url: String,
    pub inner: AdminConfig,
}

impl Deref for Config {
    type Target = AdminConfig;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<core::Config> for Config {
    fn from(config: core::Config) -> Self {
        Config {
            database_url: config.database_url.clone(),
            inner: config.admin.unwrap().clone(),
        }
    }
}
