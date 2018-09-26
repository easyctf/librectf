use std::convert::{TryFrom, TryInto};

use bcrypt;
use diesel::{self, prelude::*};
use regex::Regex;
use rocket::{
    request::{FlashMessage, Form},
    response::{Flash, Redirect, Responder, Response},
};

use db::Connection;
use models::NewUser;
use web::{responder::Either, ContextGuard, Template};

lazy_static! {
    static ref EMAIL_PATTERN: Regex = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();
}

generate_form_field!(value => RegisterEmail(pub String) {
    if !EMAIL_PATTERN.is_match(value) {
        return Err("Please enter a valid email address.".to_owned());
    }
    Ok(RegisterEmail(value.to_owned()))
});
generate_form_field!(value => RegisterPassword(String) {
    Ok(RegisterPassword(value.to_owned()))
});

#[derive(FromForm)]
struct RegisterForm {
    email: Result<RegisterEmail, String>,
    password: Result<RegisterPassword, String>,
}

impl<'a> TryFrom<&'a RegisterForm> for NewUser {
    type Error = Vec<String>;
    fn try_from(form: &'a RegisterForm) -> Result<Self, Self::Error> {
        let mut errors = Vec::new();
        let email = match &form.email {
            Ok(ref email) => email.0.clone(),
            Err(ref err) => {
                errors.push(err.clone());
                String::new()
            }
        };
        let password = match &form.password {
            Ok(ref password) => password.0.clone(),
            Err(ref err) => {
                errors.push(err.clone());
                String::new()
            }
        };
        if !errors.is_empty() {
            return Err(errors);
        }
        let password = bcrypt::hash(&password, bcrypt::DEFAULT_COST).unwrap();
        Ok(NewUser { email, password })
    }
}

#[get("/login")]
fn get_login(_ctx: ContextGuard) -> Flash<Redirect> {
    Flash::success(Redirect::to("/"), "Lol.")
}

#[post("/login")]
fn post_login(ctx: ContextGuard) -> Template {
    Template::render("user/login.html", &ctx)
}

#[get("/register")]
fn get_register(ctx: ContextGuard) -> Template {
    Template::render("user/register.html", &ctx)
}

#[post("/register", data = "<form>")]
fn post_register(db: Connection, form: Form<RegisterForm>) -> Flash<Redirect> {
    use schema::users;
    match <&RegisterForm as TryInto<NewUser>>::try_into(form.get()) {
        Ok(new_user) => {
            diesel::insert_into(users::table)
                .values(new_user)
                .execute(&*db)
                .unwrap();
        }
        Err(err) => {
            return Flash::new(Redirect::to("/user/register"), "danger", err.join("<br />"));
        }
    }
    Flash::success(
        Redirect::to("/user/login"),
        "Success! Check your email for a verification link.",
    )
}

#[get("/settings")]
fn get_settings(ctx: ContextGuard) -> Template {
    Template::render("base/index.html", &ctx)
}
