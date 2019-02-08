use core::State;
use warp::{Filter, Rejection};

use crate::session::Session;

#[derive(Default)]
pub struct Context(::tera::Context);

fn get_context() -> impl Clone + Filter<Extract = (Context,), Error = Rejection> {
    warp::ext::get::<State>()
        .and(warp::ext::get::<Session>())
        .map(|state, session| Context::default())
        .or(warp::any().map(|| Context::default()))
        .unify()
}

/// Generates all of the information needed to populate the navbar.
pub fn navbar() -> impl Clone + Filter<Extract = (), Error = Rejection> {
    get_context().map(|ctx| {}).untuple_one()
}
