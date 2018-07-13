use std::net::ToSocketAddrs;

use actix_web::App;
use failure::Error;

use Config;

pub struct OpenCTF {
    pub config: Config,
}

impl OpenCTF {
    pub fn new(config: Config) -> Result<Self, Error> {
        Ok(OpenCTF { config })
    }
    pub fn app(&self) -> Result<App, Error> {
        Ok(App::new())
    }
    pub fn bind_address(&self) -> (&str, u16) {
        (self.config.host.as_ref(), self.config.port)
    }
}
