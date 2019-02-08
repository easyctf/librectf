//! Functions associated with dealing with users and authentication.

use wtforms::Form;

use crate::db::DbConn;
use crate::models::{NewUser, User};
use crate::{Error, UserError};

/// The struct behind the form used in the login page.
#[derive(Form, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

/// Attempts to log in a user using a LoginForm. Upon success, the user struct
/// associated with that account is returned. If the user supplies bad
/// credentials, a `UserError::BadUsernameOrPassword` will be returned.
pub fn login_user(db: &DbConn, form: &LoginForm) -> Result<User, Error> {
    db.fetch_user(&form.email)
        .and_then(|user| {
            bcrypt::verify(&form.password, &user.password)
                .map(|result| (user, result))
                .map_err(Error::from)
        })
        .and_then(|(user, result)| {
            if result {
                Ok(user)
            } else {
                Err(Error::User(UserError::BadUsernameOrPassword))
            }
        })
}

/// The struct behind the form used in the register page.
#[derive(Form, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct RegisterForm {
    pub email: String,
    pub name: String,
    pub password: String,
}

/// Attempts to create a user using the given information. If the user tries
/// to create an account with an email or username that already exists, then
/// a `UserError` will be generated. (TODO: implement this)
pub fn register_user(db: &DbConn, form: &RegisterForm) -> Result<i32, Error> {
    let password = bcrypt::hash(form.password.to_owned(), bcrypt::DEFAULT_COST)?;
    let new_user = NewUser {
        email: form.email.to_owned(),
        name: form.name.clone(),
        password,
    };
    db.create_user(&new_user)
}
