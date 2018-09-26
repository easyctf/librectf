use base64::decode;
use rocket::{
    http::Cookies,
    request::{self, FromRequest, Request},
    Outcome,
};
use serde_cbor::from_slice;
use tera::Context;

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserGuard(Option<User>);

#[derive(Serialize, Default)]
struct FlashMessage(String, String);

#[derive(Serialize)]
pub struct ContextGuard {
    user: UserGuard,
    flash: Option<FlashMessage>,
    extra: Context,
}

impl<'a, 'r> FromRequest<'a, 'r> for ContextGuard {
    type Error = ();
    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = {
            let mut cookies = req.guard::<Cookies>()?;
            cookies
                .get_private("user")
                .and_then(|user| decode(user.value()).ok())
                .and_then(|user| from_slice(user.as_slice()).ok())
        };
        let flash = req
            .guard::<Option<request::FlashMessage>>()
            .map(|flash| {
                flash.map(|flash| FlashMessage(flash.name().to_owned(), flash.msg().to_owned()))
            }).map_failure(|(a, _)| (a, ()))?;
        let extra = Context::new();
        Outcome::Success(ContextGuard {
            user: UserGuard(user),
            flash,
            extra,
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = ();
    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let ctx = req.guard::<ContextGuard>()?;
        Outcome::Success(ctx.user)
    }
}
