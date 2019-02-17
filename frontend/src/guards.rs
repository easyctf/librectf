use core::{models::User, Error, UserErrorKind};
use warp::{Filter, Rejection};

use crate::session::Session;
use crate::extractors::get;

pub fn require_login() -> impl Clone + Filter<Extract = (), Error = Rejection> {
    get::<Session>()
        .and_then(|session: Session| 
            match session.get_user() {
                Some(user) => Ok(()),
                None => Err(warp::reject::not_found())
            }
        )
        .recover(|_| {
            Err(warp::reject::custom(Error::user(
                "You're not allowed to be here.",
                UserErrorKind::Forbidden,
            )))
        })
        .unify()
        .untuple_one()
}
