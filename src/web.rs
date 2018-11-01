use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use actix_web::server;
use failure::Error;
use api;

use Config;

#[derive(Clone, Debug, StructOpt)]
pub struct WebCommand {
    /// Run the API server.
    #[structopt(long = "api")]
    api: bool,
}

impl WebCommand {
    pub fn run(self, config: Config) -> Result<(), Error> {
        let addr = SocketAddrV4::new(
            Ipv4Addr::from_str(&config.bind_host).unwrap(),
            config.bind_port,
        );
        let api_cfg = config.api.clone();

        server::new(move || {
            let mut apps = Vec::new();
            let mut any = false;
            if self.api || !any {
                apps.extend(api::app(api_cfg.clone()));
            }
            apps})
            .bind(addr)
            .map(|server| server.run())?;

        Ok(())
    }
}
