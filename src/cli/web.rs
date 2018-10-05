use web;
use Config;
use Error;

#[derive(Debug, StructOpt)]
pub struct WebCommand {
    #[structopt(flatten)]
    config: Config,
}

impl WebCommand {
    pub fn run(&self) -> Result<(), Error> {
        let app = web::app(&self.config);
        Err(app.launch().into())
    }
}
