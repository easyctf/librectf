mod chal;
mod pages;
mod user;

use actix_web::{
    middleware::session::{CookieSessionBackend, SessionStorage},
    App,
};
use core::State;

pub fn router(state: State) -> App<State> {
    let config = state.get_web_config().unwrap();
    App::with_state(state.clone())
        .middleware(SessionStorage::new(
            CookieSessionBackend::signed(config.secret_key.as_bytes()).secure(false),
        )).resource("/", |r| r.get().with(self::pages::handler))
        .resource("/static/{path:.*}", |r| r.get().with(self::pages::statics))
        .scope("/chal", self::chal::scope)
        .scope("/user", self::user::scope)
}
