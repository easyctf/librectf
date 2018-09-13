use openctf::{web, Config};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Web {
    // potentially have options here?
}

impl Web {
    pub fn run(&self, config: &Config) {
        let app = web::app(config);
        app.launch();
    }
}
