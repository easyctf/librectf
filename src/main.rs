use std::net::SocketAddr;

use core::DatabaseUri;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    addr: SocketAddr,
    database_uri: DatabaseUri,
}

fn main() {
    let opt = Opt::from_args();
    let db = opt
        .database_uri
        .establish_connection()
        .expect("couldn't connect to db");
    warp::serve(frontend::routes(&db)).run(opt.addr);
}
