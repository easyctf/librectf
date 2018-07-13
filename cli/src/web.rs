use actix_web::{server, App};
use failure::Error;
use openctf::Config;

#[derive(Debug, StructOpt)]
pub(crate) struct Web {
    #[structopt(
        short = "p",
        long = "port",
        help = "The port the application should run on (overrides existing configuration)"
    )]
    port: Option<u16>,
}

impl Web {
    pub fn run(&self, config: Config) -> Result<(), Error> {
        // determine bind address
        let host = config.host.as_ref();
        let port = self.port.unwrap_or(config.port);

        server::new(|| App::new()).bind((host, port))?.run();
        Ok(())
    }
}
