use std::net::SocketAddr;
use std::sync::Arc;

use core::{State, DatabaseUri};
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
    let state = State::new(Arc::new(db));
    warp::serve(frontend::routes(state)).run(opt.addr);
}
