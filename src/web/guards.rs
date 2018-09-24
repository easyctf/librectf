use serde::{Serialize, Serializer};
use tera::{Context};
use rocket::{
    request::{self, FromRequest, Request},
    Outcome,
};

pub struct ContextGuard(Context);

impl<'a, 'r> FromRequest<'a, 'r> for ContextGuard {
    type Error = String;
    fn from_request(_req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let ctx = Context::new();
        Outcome::Success(ContextGuard(ctx))
    }
}

impl Serialize for ContextGuard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        self.0.serialize(serializer)
    }
}
