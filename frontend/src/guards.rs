use core::models::User;
use warp::{Filter, Rejection};

use crate::extractors::get;

pub fn require_login() -> impl Clone + Filter<Extract = (), Error = Rejection> {
    get::<User>().map(|_| ()).untuple_one()
}
