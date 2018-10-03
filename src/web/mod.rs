//! Helpers for the web-facing parts of the library.

mod guards;
mod routes;
mod static_files;
mod template;

use cache::HashMapCache;
use env_logger;
use rocket::{self, Rocket};

use self::guards::*;
use self::static_files::StaticFiles;
use self::template::Template;
use db::establish_connection;
use Config;
use TaskQueue;

/// This function produces an instance of the [Rocket](Rocket) app that we are building.
pub fn app(config: &Config) -> Rocket {
    env_logger::init();
    let cache = HashMapCache::new();
    let pool = establish_connection(&config.database_url);
    let tq = TaskQueue::new(cache);
    let config = config.clone();

    let rocket_env = if config.debug {
        rocket::config::Environment::Development
    } else {
        rocket::config::Environment::Production
    };
    let mut rcfg = rocket::Config::build(rocket_env)
        .address(config.bind_host.as_ref())
        .port(config.bind_port)
        .unwrap();
    rcfg.set_secret_key(config.secret_key.as_ref()).unwrap();
    
    rocket::custom(rcfg, true)
        .manage(config)
        .manage(tq)
        .manage(pool)
        .mount("/static", StaticFiles::default().into())
        .mount("/team", routes![routes::team::get_index])
        .mount(
            "/user",
            routes![
                routes::user::get_login,
                routes::user::get_logout,
                routes::user::get_register,
                routes::user::get_settings,
                routes::user::post_login,
                routes::user::post_register,
            ],
        ).mount(
            "/",
            routes![routes::base::get_index, routes::base::get_scoreboard],
        )
}
