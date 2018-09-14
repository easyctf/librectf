//! Helpers for the web-facing parts of the library.

mod guards;
mod routes;
mod static_files;

use orm::ConnectionPool;
use rocket::{self, Rocket};

use self::guards::*;
use self::routes::Template;
use self::static_files::StaticFiles;
use Config;

/// This function produces an instance of the [Rocket](Rocket) app that we are building.
pub fn app(config: &Config) -> Rocket {
    let pool = ConnectionPool::from(&config.database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/static", StaticFiles::default().into())
        .mount("/user", routes![routes::user::register])
        .mount("/user", routes![routes::user::settings])
        .mount("/", routes![routes::base::index])
}
