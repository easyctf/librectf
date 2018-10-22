extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate structopt;

mod config;

use structopt::StructOpt;

use config::FsConfig;

#[derive(Debug, StructOpt)]
struct FsCommand {
    #[structopt(flatten)]
    config: FsConfig,
}

impl FsCommand {
    pub fn run(&self) {

    }
}

fn main() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "warn");
    env_logger::Builder::from_env(env).init();

    let opt = FsCommand::from_args();
    opt.run()
}
