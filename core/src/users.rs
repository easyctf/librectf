use wtforms::Form;

use crate::db::DbConn;
use crate::models::{NewUser, User};
use crate::{Error, UserError};

#[derive(Form, Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

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

#[derive(Form, Serialize, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub name: String,
    pub password: String,
}

pub fn register_user(db: &DbConn, form: &RegisterForm) -> Result<i32, Error> {
    let password = bcrypt::hash(form.password.to_owned(), bcrypt::DEFAULT_COST)?;
    let new_user = NewUser {
        email: form.email.to_owned(),
        name: form.name.clone(),
        password,
    };
    db.create_user(&new_user)
}
