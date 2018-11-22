use actix_web::{
    self,
    middleware::{Middleware, Started},
    HttpRequest, HttpResponse,
};
use core::models::User;
use diesel::prelude::*;

use super::auth::verify_claims;
use State;

pub struct LoginRequired;

impl Middleware<State> for LoginRequired {
    fn start(&self, req: &HttpRequest<State>) -> actix_web::Result<Started> {
        let state = req.state();

        let headers = req.headers();
        let token = match headers.get("Authorization").and_then(|t| t.to_str().ok()) {
            // TODO: match this with regex
            Some(token) => if token.starts_with("Token ") {
                token.trim_left_matches("Token ")
            } else {
                token
            },
            None => return Ok(Started::Response(HttpResponse::Unauthorized().finish())),
        };

        let claims = match verify_claims(&state.get_secret_key(), token) {
            Ok(claims) => claims,
            Err(err) => {
                error!("Error decoding JWT from user: {:?}", err);
                return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
            }
        };

        let db = state.get_connection()?;
        let user = match {
            use core::schema::users::dsl::{id, users};
            users.filter(id.eq(claims.id)).first::<User>(&*db)
        } {
            Ok(user) => user,
            Err(err) => {
                error!("Error loading user from database: {:?}", err);
                return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
            }
        };

        let mut ext = req.extensions_mut();
        ext.insert(user);
        ext.insert(claims);
        Ok(Started::Done)
    }
}
