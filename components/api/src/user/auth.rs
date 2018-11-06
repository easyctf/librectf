use chrono::{serde::ts_milliseconds, DateTime, Utc};
use core::models::{NewUser, User};
use diesel::prelude::*;
use failure::{Compat, Error, Fail};
use jsonwebtoken::{Header, Validation};

use DbConn;

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginClaims {
    #[serde(with = "ts_milliseconds")]
    exp: DateTime<Utc>,
    pub id: i32,
    username: String,
    admin: bool,
}

#[derive(Debug, Fail)]
pub enum UserError {
    #[fail(display = "An account was already found with these credentials")]
    AlreadyRegistered,
    #[fail(display = "Bad username or password")]
    BadUsernameOrPassword,
    #[fail(display = "Internal server error")]
    ServerError(#[cause] Compat<Error>),
}

impl UserError {
    pub fn from<T: Into<Error>>(err: T) -> Self {
        UserError::ServerError(<T as Into<Error>>::into(err).compat())
    }
}

pub fn sign_claims(secret_key: &Vec<u8>, user: &User) -> Result<String, UserError> {
    let claim = LoginClaims {
        exp: Utc::now() + chrono::Duration::weeks(6),
        id: user.id,
        username: user.name.clone(),
        admin: user.admin,
    };
    jsonwebtoken::encode(&Header::default(), &claim, secret_key).map_err(|err| UserError::from(err))
}

pub fn verify_claims(secret_key: &Vec<u8>, token: &str) -> Result<LoginClaims, Error> {
    let validation = Validation {
        leeway: 60,
        ..Default::default()
    };
    jsonwebtoken::decode::<LoginClaims>(token, secret_key, &validation)
        .map(|data| data.claims)
        .map_err(|err| err.into())
}

/// Logs in a given user, given a database connection and the user's credentials.
///
/// It either returns a token that was generated from the successful authentication, or an [Error][1].
///
/// [1]: `failure::Error`
pub fn login_user(db: DbConn, secret_key: Vec<u8>, form: LoginForm) -> Result<String, UserError> {
    use core::schema::users::dsl::*;

    users
        .filter(email.eq(&form.email))
        .first::<User>(&*db)
        .map_err(|err| UserError::BadUsernameOrPassword)
        .and_then(|user| {
            bcrypt::verify(&form.password, &user.password)
                .map(|_| user)
                .map_err(|_| UserError::BadUsernameOrPassword)
        }).and_then(|user| sign_claims(&secret_key, &user))
}

#[derive(Deserialize)]
pub struct RegisterForm {
    email: String,
    username: String,
    password: String,
}

impl RegisterForm {
    fn into_new_user(self) -> Result<NewUser, UserError> {
        Ok(NewUser {
            email: self.email,
            name: self.username,
            password: bcrypt::hash(&self.password, bcrypt::DEFAULT_COST)
                .map_err(|err| UserError::from(err))?,
        })
    }
}

pub fn register_user(db: DbConn, form: RegisterForm) -> Result<(), UserError> {
    use core::schema::users;
    use diesel::result::{DatabaseErrorKind::UniqueViolation, Error::DatabaseError};

    let new_user = form.into_new_user()?;
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(&*db)
        .map_err(|err| match err {
            DatabaseError(UniqueViolation, _) => UserError::AlreadyRegistered,
            _ => UserError::from(err),
        }).map(|_| {
            // send an email
        })
}
