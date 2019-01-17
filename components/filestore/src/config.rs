use std::ops::Deref;

use actix_web::{FromRequest, HttpRequest};
use core::{config::FilestoreConfig, Error, State};

pub struct Config(FilestoreConfig);

impl Deref for Config {
    type Target = FilestoreConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromRequest<State> for Config {
    type Config = ();
    type Result = Result<Config, Error>;

    fn from_request(req: &HttpRequest<State>, _: &Self::Config) -> Self::Result {
        let state = req.state();
        state
            .get_filestore_config()
            .map(|cfg| Config(cfg.clone()))
            .ok_or_else(|| unreachable!())
    }
}
