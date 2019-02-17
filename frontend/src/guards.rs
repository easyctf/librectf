use core::{models::User, Error, UserErrorKind};
use warp::{Filter, Rejection};

use crate::extractors::get;
use crate::session::Session;

pub fn require_login() -> impl Clone + Filter<Extract = (User,), Error = Rejection> {
    get::<Session>()
        .and_then(|session: Session| match session.get_user() {
            Some(user) => Ok(user.clone()),
            None => Err(warp::reject::not_found()),
        })
        .recover(|_| {
            Err(warp::reject::custom(Error::user(
                "You're not logged in.",
                UserErrorKind::Forbidden,
            )))
        })
        .unify()
}
