use chrono::{serde::ts_milliseconds, DateTime, Utc};
use core::models::User;
use diesel::prelude::*;
use failure::{Compat, Error, Fail};
use jsonwebtoken::Header;

use DbConn;

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginClaim {
    #[serde(with = "ts_milliseconds")]
    exp: DateTime<Utc>,
    pub id: i32,
    username: String,
    admin: bool,
}

#[derive(Debug, Fail)]
pub enum LoginError {
    #[fail(display = "Bad username or password")]
    BadUsernameOrPassword,
    #[fail(display = "Internal server error")]
    ServerError(#[cause] Compat<Error>),
}

pub fn sign_claims(secret_key: &Vec<u8>, user: &User) -> Result<String, LoginError> {
    let claim = LoginClaim {
        exp: Utc::now() + chrono::Duration::weeks(6),
        id: user.id,
        username: user.name.clone(),
        admin: user.admin,
    };
    jsonwebtoken::encode(&Header::default(), &claim, secret_key).map_err(|err| {
        use jsonwebtoken::errors::Error as JwtError;
        LoginError::ServerError(<JwtError as Into<Error>>::into(err).compat())
    })
}

/// Logs in a given user, given a database connection and the user's credentials.
///
/// It either returns a token that was generated from the successful authentication, or an [Error][1].
///
/// [1]: `failure::Error`
pub fn login_user(db: DbConn, secret_key: Vec<u8>, form: LoginForm) -> Result<String, LoginError> {
    use core::schema::users::dsl::*;

    users
        .filter(email.eq(&form.email))
        .first::<User>(&*db)
        .map_err(|err| LoginError::BadUsernameOrPassword)
        .and_then(|user| {
            bcrypt::verify(&form.password, &user.password)
                .map(|_| user)
                .map_err(|_| LoginError::BadUsernameOrPassword)
        }).and_then(|user| sign_claims(&secret_key, &user))
}
