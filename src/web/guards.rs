use rocket::{
    request::{self, FromRequest, Request},
    Outcome,
};

#[derive(Default)]
pub struct UserGuard {}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = String;
    fn from_request(_req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(UserGuard::default())
    }
}
