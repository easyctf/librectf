use web::{self, WebConfig};
use Error;

#[derive(Debug, StructOpt)]
pub struct WebCommand {
    #[structopt(flatten)]
    config: WebConfig,
}

impl WebCommand {
    pub fn run(&self) -> Result<(), Error> {
        web::run(self.config.clone())
    }
}
