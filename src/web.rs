use std::net::{Ipv4Addr, SocketAddrV4};
use std::str::FromStr;

use actix_web::server;
// use api;
use core::{Config, State};
use failure::Error;

#[derive(Clone, Debug, StructOpt)]
pub struct WebCommand {
    // /// Run the API server.
    // #[structopt(long = "api")]
    // api: bool,

    /// Run the static file server.
    #[structopt(long = "filestore")]
    filestore: bool,
}

impl WebCommand {
    pub fn run(self, config: &Config) -> Result<(), Error> {
        // TODO: clean this up
        let web = match &config.web {
            Some(cfg) => cfg.clone(),
            None => bail!("Missing web config section."),
        };
        println!("{:?} {:?}", self, config);

        let addr = SocketAddrV4::new(Ipv4Addr::from_str(&web.bind_host).unwrap(), web.bind_port);

        let state = State::from(&config.clone());
        server::new(move || {
            // let api = web.api.as_ref();
            let filestore = web.filestore.as_ref();
            vec![
                Some(frontend::app(state.clone()).unwrap()),
                // api.and_then(|_| api::app(state.clone()).ok()),
                filestore.and_then(|_| filestore::app(state.clone()).ok()),
            ].into_iter()
            .filter_map(|app| app)
            .collect::<Vec<_>>()
        }).bind(addr)
        .map(|server| server.run())?;

        Ok(())
    }
}
