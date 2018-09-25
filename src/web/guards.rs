use rocket::{
    request::{self, FromRequest, Request},
    Outcome,
};
use serde::ser::{Serialize, SerializeStruct, Serializer};
use tera::Context;

#[derive(Serialize, Default)]
struct UserGuard {}

#[derive(Serialize, Default)]
struct FlashMessage(String, String);

#[derive(Serialize)]
pub struct ContextGuard {
    user: Option<UserGuard>,
    flash: Option<FlashMessage>,
    extra: Context,
}

impl<'a, 'r> FromRequest<'a, 'r> for ContextGuard {
    type Error = String;
    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = None;
        let flash = req
            .guard::<Option<request::FlashMessage>>()
            .map(|flash| {
                flash.map(|flash| FlashMessage(flash.name().to_owned(), flash.msg().to_owned()))
            }).map_failure(|(a, _)| (a, String::new()))?;
        let extra = Context::new();
        Outcome::Success(ContextGuard { user, flash, extra })
    }
}
