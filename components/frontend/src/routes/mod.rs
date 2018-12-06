mod pages;

use actix_web::App;
use core::State;

pub fn router(state: State) -> App<State> {
    App::with_state(state).resource("/", |r| r.get().with(self::pages::handler))
}
