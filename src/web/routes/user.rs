use std::convert::{TryFrom, TryInto};

use base64::encode;
use bcrypt;
use diesel::{self, prelude::*};
use regex::Regex;
use rocket::{
    http::{Cookie, Cookies},
    request::Form,
    response::{Flash, Redirect},
};
use serde_cbor::to_vec;

use db::Connection;
use models::{NewUser, User};
use web::{guards::UserGuard, ContextGuard, Template};
use INTERNAL_SERVER_ERROR_MESSAGE;

lazy_static! {
    static ref USERNAME_PATTERN: Regex = Regex::new(r"[A-Za-z_][A-Za-z0-9_]{2,}").unwrap();
    static ref EMAIL_PATTERN: Regex = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();
}

generate_form_field!(value => UsernameField(pub String) {
    if !USERNAME_PATTERN.is_match(&value) {
        return Err(format!("Invalid username (must be 3-20 chars and begin with a non-numeric character)."));
    }
    Ok(UsernameField(value.to_owned()))
});

generate_form_field!(value => EmailField(pub String) {
    if !EMAIL_PATTERN.is_match(&value) {
        return Err(format!("Invalid email."));
    }
    Ok(EmailField(value.to_owned()))
});

generate_form_field!(value => RegisterPasswordField(String) {
    let hashed = bcrypt::hash(&value, bcrypt::DEFAULT_COST).unwrap();
    Ok(RegisterPasswordField(hashed))
});

generate_form_field!(value => LoginPasswordField(String) {
    Ok(LoginPasswordField(value))
});

generate_form!(RegisterForm => NewUser {
    name = username: UsernameField,
    email = email: EmailField,
    password = password: RegisterPasswordField,
});

struct _LoginForm {
    email: String,
    password: String,
}

generate_form!(LoginForm => _LoginForm {
    email = email: EmailField,
    password = password: LoginPasswordField,
});

#[get("/login")]
fn get_login(ctx: ContextGuard) -> Template {
    Template::render("user/login.html", &ctx)
}

#[post("/login", data = "<form>")]
fn post_login(
    db: Connection,
    form: Form<LoginForm>,
    mut cookies: Cookies,
) -> Result<Redirect, Flash<Redirect>> {
    use schema::users::dsl::*;
    match <&LoginForm as TryInto<_LoginForm>>::try_into(form.get())
        .and_then(|form| {
            users
                .filter(email.eq(&form.email))
                .first::<User>(&*db)
                .map(|user| (form, user))
                .map_err(|err| {
                    error!("Diesel error on user::post_login: {}", err);
                    vec![INTERNAL_SERVER_ERROR_MESSAGE.to_owned()]
                })
        }).and_then(
            |(form, user)| match bcrypt::verify(&form.password, &user.password) {
                Ok(true) => {
                    let user_guard = UserGuard { name: user.name };
                    to_vec(&user_guard)
                        .map_err(|_| Vec::new())
                        .map(|vec| encode(vec.as_slice()))
                }
                Ok(false) => Err(vec![
                    "Incorrect username or password, please try again!".to_owned(),
                ]),
                Err(err) => {
                    error!("Bcrypt error on user::post_login: {}", err);
                    Err(vec![INTERNAL_SERVER_ERROR_MESSAGE.to_owned()])
                }
            },
        ).and_then(|encoded| {
            cookies.add_private(Cookie::new("user", encoded));
            Ok(())
        }) {
        Ok(_) => Ok(Redirect::to("/team")),
        Err(err) => Err(Flash::new(
            Redirect::to("/user/login"),
            "danger",
            err.join("<br />"),
        )),
    }
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
                ) => vec!["This username or email has already been taken.".to_owned()],
                _ => {
                    error!("Diesel error on user::post_register: {}", err);
                    vec![INTERNAL_SERVER_ERROR_MESSAGE.to_owned()]
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
