use actix_web::App;

use super::{APIMiddleware, State};

pub fn app(state: State) -> App<State> {
    App::with_state(state).middleware(APIMiddleware)
}
