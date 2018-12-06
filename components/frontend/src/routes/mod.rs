mod pages;
mod user;

use actix_web::{middleware::csrf::CsrfFilter, App};
use core::State;

pub fn router(state: State) -> App<State> {
    App::with_state(state)
        .middleware(CsrfFilter::new())
        .resource("/", |r| r.get().with(self::pages::handler))
        .resource("/static/{path:.*}", |r| r.get().with(self::pages::statics))
        .scope("/user", self::user::scope)
}
