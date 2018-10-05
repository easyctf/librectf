use base64::decode;
use rocket::{
    http::Cookies,
    request::{self, FromRequest, Request},
    Outcome,
};
use serde_cbor::from_slice;

#[derive(Serialize, Deserialize, Default)]
pub struct User {
    pub name: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct UserGuard(Option<User>);

impl<'a, 'r> FromRequest<'a, 'r> for UserGuard {
    type Error = ();
    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let user = {
            let mut cookies = req.guard::<Cookies>()?;
            cookies
                .get_private("user")
                .and_then(|user| decode(user.value()).ok())
                .and_then(|user| from_slice(user.as_slice()).ok())
        };
        Outcome::Success(UserGuard(user))
    }
}
