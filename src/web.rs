use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use actix_web::server;
use api;
use failure::Error;

use Config;

#[derive(Clone, Debug, StructOpt)]
pub struct WebCommand {
    /// Run the API server.
    #[structopt(long = "api")]
    api: bool,

    /// Run the static file server.
    #[structopt(long = "filestore")]
    filestore: bool,
}

impl WebCommand {
    pub fn run(self, config: Config) -> Result<(), Error> {
        // TODO: clean this up
        let addr = SocketAddrV4::new(
            Ipv4Addr::from_str(&config.bind_host).unwrap(),
            config.bind_port,
        );
        let api_cfg = config.api.clone();
        let filestore_cfg = config.filestore.clone();

        server::new(move || {
            let mut apps = Vec::new();
            let mut any = false;
            if self.filestore {
                apps.extend(filestore::app(filestore_cfg.clone()));
                any = true;
            }
            if self.api || !any {
                let cfg = api_cfg.expect("Missing api config.");
                apps.extend(api::app(cfg));
            }
            apps
        }).bind(addr)
        .map(|server| server.run())?;

        Ok(())
    }
}
