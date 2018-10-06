use actix_web::{Form, HttpRequest};
use bcrypt;
use diesel::{self, RunQueryDsl};
use serde::Serialize;

use super::{errors::WebError, DbConn, JsonResult, State};
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

pub fn register((form, db): (Form<RegisterForm>, DbConn)) -> JsonResult<String, String> {
    use schema::users;
    let new_user: NewUser = match form.into_inner().into() {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to register: {:?}", err);
            return JsonResult::Err(err.to_string());
        }
    };
    diesel::insert_into(users::table)
        .values(new_user)
        .execute(&*db)
        .map(|_| JsonResult::<String, _>::ok("Registered successfully".to_owned()))
        .unwrap_or_else(|err| {
            error!("Failed to insert registration into db: {:?}", err);
            JsonResult::<_, String>::err(
                "Failed to complete registration, contact an admin.".to_owned(),
            )
        })
}

impl Into<Result<NewUser, WebError>> for RegisterForm {
    fn into(self) -> Result<NewUser, WebError> {
        Ok(NewUser {
            email: self.email,
            name: self.name,
            password: bcrypt::hash(&self.password, bcrypt::DEFAULT_COST)?,
        })
    }
}
