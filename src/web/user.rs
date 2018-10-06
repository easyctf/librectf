use actix_web::{Form, HttpRequest};
use diesel::{self, RunQueryDsl};

use super::{errors::WebError, DbConn, State};
use models::NewUser;

pub fn login(_: &HttpRequest<State>) -> String {
    String::new()
}

#[derive(Deserialize)]
pub struct RegisterForm {
    email: String,
    name: String,
    password: String,
}

pub fn register((form, db): (Form<RegisterForm>, DbConn)) -> String {
    use schema::users;
    let new_user: NewUser = match form.into_inner().into() {
        Ok(user) => user,
        Err(_) => panic!("shiet"),
    };
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(&*db);
    String::new()
}

impl Into<Result<NewUser, WebError>> for RegisterForm {
    fn into(self) -> Result<NewUser, WebError> {
        Ok(NewUser {
            email: self.email,
            name: self.name,
            password: self.password,
        })
    }
}
