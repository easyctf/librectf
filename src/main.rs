use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use backtrace::Backtrace;
use core::{DatabaseUri, Error, State};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(flatten)]
    command: Commands,
}

#[derive(StructOpt)]
enum Commands {
    /// Run migrations
    #[structopt(name = "migrate")]
    Migrate(MigrateOpts),

    /// Start the web server
    #[structopt(name = "run")]
    Run(RunOpts),
}

#[derive(StructOpt)]
struct MigrateOpts {
    /// The database uri (mysql:// , postgres:// , sqlite://)
    #[structopt(long = "database-uri")]
    database_uri: DatabaseUri,
}

#[derive(StructOpt)]
struct RunOpts {
    /// The database uri (mysql:// , postgres:// , sqlite://)
    #[structopt(long = "database-uri")]
    database_uri: DatabaseUri,

    /// The address to bind to
    #[structopt(long = "bind-addr")]
    addr: SocketAddr,

    /// Secret key
    #[structopt(long = "secret-key")]
    secret_key: String,
}

fn run(opt: Opt) -> Result<(), Error> {
    match opt.command {
        Commands::Migrate(opts) => {
            let db = opts
                .database_uri
                .establish_connection()
                .expect("couldn't connect to db");

            let conn = db.get().expect("couldn't get connection");
            let res = conn.run_migrations(&mut io::stdout());
            println!("{:?}", res);
        }
        Commands::Run(opts) => {
            let db = opts
                .database_uri
                .establish_connection()
                .expect("couldn't connect to db");

            let state = State::new(opts.secret_key, Arc::new(db));
            warp::serve(frontend::routes(state)).run(opts.addr);
        }
    }
    Ok(())
}

fn main() {
    env_logger::init();
    let backtrace = Backtrace::new();
    let opt = Opt::from_args();
    match run(opt) {
        Ok(_) => (),
        Err(err) => {
            println!("backtrace: {:?}", backtrace);
        }
    }
}
