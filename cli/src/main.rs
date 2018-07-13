//! # OpenCTF CLI

#[macro_use]
extern crate structopt;

mod web;

use structopt::StructOpt;

use web::Web;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "web")]
    Web(Web),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "openctf", about = "Flexible and powerful CTF framework.")]
struct OpenCTF {
    #[structopt(subcommand)]
    cmd: Command,
}

fn main() {
    let opt = OpenCTF::from_args();
    println!("{:?}", opt);
}
