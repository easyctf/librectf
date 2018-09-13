use structopt::StructOpt;
use openctf::web;

#[derive(Debug, StructOpt)]
pub struct Web {
    // potentially have options here?
}

impl Web {
    pub fn run(&self) {
        let app = web::app();
        app.launch();
    }
}
