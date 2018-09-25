//! Helpers for the web-facing parts of the library.

mod guards;
mod routes;
mod static_files;
mod template;

use env_logger;
use rocket::{self, Rocket};

use self::guards::*;
use self::static_files::StaticFiles;
use self::template::Template;
use db::establish_connection;
use Config;

/// This function produces an instance of the [Rocket](Rocket) app that we are building.
pub fn app(config: &Config) -> Rocket {
    env_logger::init();
    let pool = establish_connection(&config.database_url);
    rocket::ignite()
        .manage(pool)
        .mount("/static", StaticFiles::default().into())
        .mount(
            "/user",
            routes![
                routes::user::get_login,
                routes::user::get_register,
                routes::user::get_settings,
                routes::user::post_login,
                routes::user::post_register,
            ],
        ).mount("/", routes![routes::base::get_index])
}
