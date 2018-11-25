use chrono::{serde::ts_milliseconds, DateTime, Utc};
use core::models::{NewUser, User};
use diesel::{
    prelude::*,
    result::{
        DatabaseErrorKind::UniqueViolation,
        Error::{DatabaseError, RollbackTransaction},
    },
};
use failure::{Compat, Error, Fail};
use jsonwebtoken::{Header, Validation};

use DbConn;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
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

pub fn sign_claims(secret_key: &[u8], user: &User) -> Result<String, UserError> {
    let claim = LoginClaims {
        exp: Utc::now() + chrono::Duration::weeks(6),
        id: user.id,
        username: user.name.clone(),
        admin: user.admin,
    };
    jsonwebtoken::encode(&Header::default(), &claim, secret_key).map_err(UserError::from)
}

pub fn verify_claims(secret_key: &[u8], token: &str) -> Result<LoginClaims, Error> {
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
pub fn login_user(
    db: DbConn,
    secret_key: &[u8],
    form: LoginForm,
) -> Result<(User, String), UserError> {
    use core::schema::users::dsl::*;

    users
        .filter(email.eq(&form.email))
        .first::<User>(&*db)
        .map_err(|_| UserError::BadUsernameOrPassword)
        .and_then(|user| {
            bcrypt::verify(&form.password, &user.password)
                .map(|_| user)
                .map_err(|_| UserError::BadUsernameOrPassword)
        }).and_then(|user| sign_claims(secret_key, &user).map(|claims| (user, claims)))
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub username: String,
    pub password: String,
}

impl RegisterForm {
    fn into_new_user(self) -> Result<NewUser, UserError> {
        Ok(NewUser {
            email: self.email,
            name: self.username,
            password: bcrypt::hash(&self.password, bcrypt::DEFAULT_COST)
                .map_err(UserError::from)?,
        })
    }
}

pub fn register_user(
    db: DbConn,
    secret_key: &[u8],
    form: RegisterForm,
) -> Result<(User, String), UserError> {
    let new_user = form.into_new_user()?;
    db.transaction(|| {
        if let Err(err) = {
            use core::schema::users;
            diesel::insert_into(users::table)
                .values(new_user)
                .execute(&*db)
        } {
            error!("Diesel error on register: {}", err);
            return Err(RollbackTransaction);
        }

        let user = match {
            use core::schema::users::dsl::{id, users};
            users.order_by(id.desc()).first::<User>(&*db)
        } {
            Ok(user) => user,
            Err(err) => {
                error!("Diesel error on register: {}", err);
                return Err(RollbackTransaction);
            }
        };

        Ok(user)
    }).map_err(|err| match err {
        DatabaseError(UniqueViolation, _) => UserError::AlreadyRegistered,
        _ => UserError::from(err),
    }).and_then(|user| {
        // login the user
        let claims = sign_claims(secret_key, &user);

        // send an email

        claims.map(|claims| (user, claims))
    })
}
