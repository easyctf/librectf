pub mod auth;
mod middleware;

use actix_web::{HttpResponse, Json};
use bcrypt;
use diesel::{self, prelude::*};
use failure::Error;

use super::DbConn;
use core::models::NewUser;

pub use self::auth::LoginClaim;
pub use self::middleware::LoginRequired;

#[derive(Deserialize)]
struct RegisterForm {
    email: String,
    username: String,
    password: String,
}

fn register((form, db): (Json<RegisterForm>, DbConn)) -> HttpResponse {
    use core::schema::users;

    let new_user: NewUser = match form.into_inner().into() {
        Ok(user) => user,
        Err(err) => {
            error!("Failed to register: {:?}", err);
            return HttpResponse::BadRequest().json(err.to_string());
        }
    };

    diesel::insert_into(users::table)
        .values(new_user)
        .execute(&*db)
        .map(|_| HttpResponse::Ok().json("Registration successful."))
        .unwrap_or_else(|err| {
            error!("Diesel error on user/register: {}", err);
            HttpResponse::BadRequest().json("Failed to complete registration.")
        })
}

// TODO: this should be a ValidationError instead of WebError
impl Into<Result<NewUser, Error>> for RegisterForm {
    fn into(self) -> Result<NewUser, Error> {
        Ok(NewUser {
            email: self.email,
            name: self.username,
            password: bcrypt::hash(&self.password, bcrypt::DEFAULT_COST)?,
        })
    }
}

fn get_settings(db: DbConn) -> HttpResponse {
    HttpResponse::Ok().json("")
}

fn post_settings(db: DbConn) -> HttpResponse {
    HttpResponse::Ok().json("")
}
