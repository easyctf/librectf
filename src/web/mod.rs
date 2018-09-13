//! Helpers for the web-facing parts of the library.

mod routes;
mod static_files;

use rocket::{self, Rocket};

use self::static_files::StaticFiles;

/// This function produces an instance of the [Rocket](Rocket) app that we are building.
pub fn app() -> Rocket {
    rocket::ignite()
        .mount("/static", StaticFiles::default().into())
        .mount("/", routes![routes::base::index])
}
