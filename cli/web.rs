use std::env;
use std::sync::Arc;

use actix_web::server;
use env_logger;
use failure::Error;
use openctf::{Config, OpenCTF};

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
        let ctf = Arc::new(OpenCTF::new(config)?);

        let addr_ref = ctf.clone();
        let mut addr = addr_ref.bind_address();
        if let Some(port) = self.port {
            addr.1 = port;
        }

        env::set_var("RUST_LOG", "actix_web=info");
        env_logger::init();
        server::new(move || ctf.app()).bind(addr)?.run();
        Ok(())
    }
}
