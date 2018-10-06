mod forms;

use std::convert::TryInto;

use base64::encode;
use bcrypt;
use diesel::{self, prelude::*};
use rocket::{
    http::{Cookie, Cookies},
    request::Form,
    response::{Flash, Redirect},
};
use serde_cbor::to_vec;

use db::Connection;
use models::{NewUser, User};
use web::guards::User as CUser;
use INTERNAL_SERVER_ERROR_MESSAGE;

use self::forms::{LoginForm, RegisterForm, _LoginForm};

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
                    let user = CUser { name: user.name };
                    to_vec(&user)
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

#[get("/logout")]
fn get_logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user"));
    Flash::success(Redirect::to("/"), "Successfully logged out!")
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
