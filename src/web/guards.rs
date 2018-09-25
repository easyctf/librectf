use rocket::{
    request::{self, FromRequest, Request},
    Outcome,
};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use tera::Context;

#[derive(Serialize, Default)]
struct UserGuard {}

pub struct ContextGuard {
    user: Option<UserGuard>,
    extra: Context,
}

impl<'a, 'r> FromRequest<'a, 'r> for ContextGuard {
    type Error = String;
    fn from_request(_req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = None;
        let extra = Context::new();
        Outcome::Success(ContextGuard { user, extra })
    }
}

impl Serialize for ContextGuard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("context", 2)?;
        s.serialize_field("logged_in", &self.user.is_some())?;
        s.serialize_field("user", &self.user)?;
        s.serialize_field("extra", &self.extra)?;
        s.end()
    }
}
