//! Helpers for the web-facing parts of the library.

mod routes;

use rocket::{self, Rocket};

/// This function produces an instance of the [Rocket](Rocket) app that we are building.
pub fn app() -> Rocket {
    rocket::ignite().mount("/", routes![routes::base::index])
}
