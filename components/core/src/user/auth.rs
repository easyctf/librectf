use diesel::{
    prelude::*,
    result::{
        DatabaseErrorKind::UniqueViolation,
        Error::{DatabaseError, RollbackTransaction},
    },
    Connection,
};
use actix_web;
use failure::{Compat, Error, Fail};

use db::Connection as DbConn;
use models::{NewUser, User};

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub user: String,
    pub password: String,
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

impl From<actix_web::Error> for UserError {
    fn from(err: actix_web::Error) -> Self {
        UserError::from(format_err!("{}", err))
    }
}

/// Logs in a given user, given a database connection and the user's credentials.
///
/// It either returns a token that was generated from the successful authentication, or an [Error][1].
///
/// [1]: `failure::Error`
pub fn login_user(db: DbConn, form: LoginForm) -> Result<User, UserError> {
    use schema::users::dsl::*;

    users
        .filter(email.eq(&form.user))
        .or_filter(name.eq(&form.user))
        .first::<User>(&*db)
        // TODO: match this
        .map_err(|_| UserError::BadUsernameOrPassword)
        .and_then(|user| {
            bcrypt::verify(&form.password, &user.password)
                .map_err(UserError::from)
                .and_then(|correct| {
                    if correct {
                        Ok(user)
                    } else {
                        Err(UserError::BadUsernameOrPassword)
                    }
                })
        })
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

pub fn register_user(db: DbConn, form: RegisterForm) -> Result<User, UserError> {
    let new_user = form.into_new_user()?;
    db.transaction(|| {
        if let Err(err) = {
            use schema::users;
            diesel::insert_into(users::table)
                .values(new_user)
                .execute(&*db)
        } {
            error!("Diesel error on register: {}", err);
            return Err(RollbackTransaction);
        }

        let user = match {
            use schema::users::dsl::{id, users};
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
    })
}
