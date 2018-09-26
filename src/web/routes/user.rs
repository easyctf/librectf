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

generate_form_field!(value => RegisterEmail(pub String) {
    // TODO: validate email to some general regex
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
    match <&RegisterForm as TryInto<NewUser>>::try_into(form.get()).and_then(|new_user| {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(&*db)
            .map_err(|err| match err {
                diesel::result::Error::DatabaseError(
                    diesel::result::DatabaseErrorKind::UniqueViolation,
                    _,
                ) => vec!["This email has already been taken.".to_owned()],
                _ => {
                    error!("Diesel error on user::post_register: {}", err);
                    vec!["Internal server error, please contact the webmaster.".to_owned()]
                }
            })
    }) {
        Ok(_) => Flash::success(
            Redirect::to("/user/login"),
            "Success! Check your email for a verification link.",
        ),
        Err(err) => Flash::new(Redirect::to("/user/register"), "danger", err.join("<br />")),
    }
}

#[get("/settings")]
fn get_settings(ctx: ContextGuard) -> Template {
    Template::render("base/index.html", &ctx)
}
