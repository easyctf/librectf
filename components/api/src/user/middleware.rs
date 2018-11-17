use actix_web::{
    self,
    middleware::{Middleware, Started},
    HttpRequest, HttpResponse,
};

use super::auth::verify_claims;
use State;

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

        verify_claims(&state.get_secret_key(), token)
            .map(|claims| {
                let mut ext = req.extensions_mut();
                ext.insert(claims);
                Ok(Started::Done)
            }).unwrap_or_else(|err| {
                error!("Error decoding JWT from user: {:?}", err);
                return Ok(Started::Response(
                    HttpResponse::Forbidden().json("access denied 1"),
                ));
            })
    }
}
