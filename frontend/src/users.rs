use core::{
    models::User,
    users::{LoginForm, RegisterForm},
    Error, State,
};
use http::uri::Uri;
use warp::Filter;
use wtforms::Form;

use crate::extractors::{get_context, navbar, Context};
use crate::render::render_template;
use crate::session::Session;

pub fn get_login() -> Resp!() {
    warp::path::end()
        .and(navbar())
        .and(get_context())
        .and_then(|ctx: Context| render_template("users/login.html", ctx.into()))
}

pub fn post_login() -> Resp!() {
    warp::body::form()
        .and_then(|form: LoginForm| {
            form.validate()
                .map_err(Error::from)
                .map_err(warp::reject::custom)
        })
        .and(warp::ext::get::<State>())
        .and_then(|form: LoginForm, state: State| {
            state
                .get_connection()
                .and_then(|conn| core::users::login_user(&conn, &form))
                .map_err(warp::reject::custom)
        })
        .and(warp::ext::get::<Session>())
        .map(|user: User, mut session: Session| {
            session.user_id = Some(user.id);
            warp::ext::set::<Session>(session);
            warp::redirect::redirect(Uri::from_static("/users/profile"))
        })
}

pub fn get_profile() -> Resp!() {
    warp::path::end()
        .and(navbar())
        .and(get_context())
        .and_then(|ctx: Context| render_template("users/profile.html", ctx.into()))
}

pub fn get_register() -> Resp!() {
    warp::path::end()
        .and(navbar())
        .and(get_context())
        .and_then(|ctx: Context| render_template("users/register.html", ctx.into()))
}

pub fn post_register() -> Resp!() {
    warp::body::form()
        .and_then(|form: RegisterForm| {
            form.validate()
                .map_err(Error::from)
                .map_err(warp::reject::custom)
        })
        .and(warp::ext::get::<State>())
        .and_then(|form: RegisterForm, state: State| {
            state
                .get_connection()
                .and_then(|conn| core::users::register_user(&conn, &form))
                .map_err(warp::reject::custom)
        })
        .and(warp::ext::get::<Session>())
        .map(|user_id: i32, mut session: Session| {
            session.user_id = Some(user_id);
            warp::ext::set::<Session>(session);
            warp::redirect::redirect(Uri::from_static("/users/profile"))
        })
}
