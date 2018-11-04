use actix_web::{
    self,
    middleware::{Middleware, Started},
    HttpRequest, HttpResponse,
};
use jsonwebtoken::Validation;

use super::{super::State, LoginClaim};

pub struct LoginRequired;

impl Middleware<State> for LoginRequired {
    fn start(&self, req: &HttpRequest<State>) -> actix_web::Result<Started> {
        let state = req.state();

        let headers = req.headers();
        let token = match headers.get("Authorization").and_then(|t| t.to_str().ok()) {
            Some(token) => if token.starts_with("Token ") {
                token.trim_left_matches("Token ")
            } else {
                token
            },
            None => {
                return Ok(Started::Response(
                    HttpResponse::Forbidden().json("access denied 1"),
                ))
            }
        };

        // TODO: don't unwrap here
        let validation = Validation {
            leeway: 60,
            ..Default::default()
        };
        let decoded =
            match jsonwebtoken::decode::<LoginClaim>(token, &state.get_secret_key(), &validation) {
                Ok(claims) => claims,
                err => {
                    error!("Error decoding JWT from user: {:?}", err);
                    return Ok(Started::Response(
                        HttpResponse::Forbidden().json("access denied 1"),
                    ));
                }
            };

        let mut ext = req.extensions_mut();
        ext.insert(decoded.claims);

        Ok(Started::Done)
    }
}

#[test]
fn test_login_required() {
    panic!();
}
