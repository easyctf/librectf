mod pages;
mod user;

use actix_web::{
    middleware::session::{CookieSessionBackend, SessionStorage},
    App,
};
use core::State;

pub fn router(state: State) -> App<State> {
    App::with_state(state)
        .middleware(SessionStorage::new(
            CookieSessionBackend::signed(&[0; 32]).secure(false),
        )).resource("/", |r| r.get().with(self::pages::handler))
        .resource("/static/{path:.*}", |r| r.get().with(self::pages::statics))
        .scope("/user", self::user::scope)
}
