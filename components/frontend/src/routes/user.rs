use actix_web::{Form, HttpRequest, HttpResponse, Scope};
use core::{
    user::auth::{login_user, LoginForm},
    State,
};
use tera::Context;

use Request;

pub fn scope(app: Scope<State>) -> Scope<State> {
    app.resource("/login", |r| {
        r.get().with(get_login);
        r.post().with(post_login)
    }).resource("/register", |r| r.get().with(get_register))
}

fn get_login(req: Request) -> HttpResponse {
    req.state
        .render("user/login.html", &req.ctx)
        .map(|content| HttpResponse::Ok().body(content))
        .map(|err| err.into())
        .unwrap_or_else(|err| {
            error!("Error during Tera rendering: {}", err);
            HttpResponse::InternalServerError().finish()
        })
}

fn post_login((req, form): (Request, Form<LoginForm>)) -> HttpResponse {
    let db = req.state.get_connection().unwrap();
    let form = form.into_inner();

    login_user(db, form)
        .map(|user| HttpResponse::Ok().body(format!("{:?}", user)))
        .unwrap_or_else(|err| HttpResponse::InternalServerError().finish())
}

fn get_register(req: Request) -> HttpResponse {
    req.state
        .render("user/register.html", &req.ctx)
        .map(|content| HttpResponse::Ok().body(content))
        .map(|err| err.into())
        .unwrap_or_else(|err| {
            error!("Error during Tera rendering: {}", err);
            HttpResponse::InternalServerError().finish()
        })
}
