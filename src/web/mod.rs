//! Helpers for the web-facing parts of the library.

mod guards;
mod routes;

use env_logger;
use rocket::{self, Rocket};
use task_queue::TaskQueue;

use self::guards::*;
use db::establish_connection;
use Config;

/// This function produces an instance of the [Rocket](Rocket) app that we are building.
pub fn app(config: &Config) -> Rocket {
    env_logger::init();
    let pool = establish_connection(&config.database_url);
    let tq = TaskQueue::new();
    let config = config.clone();

    let rocket_cfg = {
        let rocket_env = if config.debug {
            rocket::config::Environment::Development
        } else {
            rocket::config::Environment::Production
        };
        let mut cfg = rocket::Config::build(rocket_env)
            .address(config.bind_host.as_ref())
            .port(config.bind_port)
            .unwrap();
        cfg.set_secret_key(config.secret_key.as_ref()).unwrap();
        cfg
    };

    rocket::custom(rocket_cfg, true)
        .manage(config)
        .manage(tq)
        .manage(pool)
        .mount("/team", routes![])
        .mount(
            "/user",
            routes![
                routes::user::get_logout,
                routes::user::post_login,
                routes::user::post_register,
            ],
        ).mount("/", routes![])
}
