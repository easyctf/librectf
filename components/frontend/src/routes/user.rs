use actix_web::{Form, HttpRequest, HttpResponse, Scope};
use core::{
    user::auth::{login_user, LoginForm},
    State,
};
use tera::Context;

pub fn scope(app: Scope<State>) -> Scope<State> {
    app.resource("/login", |r| {
        r.get().with(get_login);
        r.post().with(post_login)
    }).resource("/register", |r| r.get().with(get_register))
}

fn get_login(req: HttpRequest<State>) -> HttpResponse {
    let state = req.state();
    let ctx = Context::new();

    state
        .render("user/login.html", &ctx)
        .map(|content| HttpResponse::Ok().body(content))
        .map(|err| err.into())
        .unwrap_or_else(|err| {
            error!("Error during Tera rendering: {}", err);
            HttpResponse::InternalServerError().finish()
        })
}

fn post_login((req, form): (HttpRequest<State>, Form<LoginForm>)) -> HttpResponse {
    let state = req.state();
    let cfg = state.get_web_config().unwrap();
    let db = state.get_connection().unwrap();
    let form = form.into_inner();

    login_user(db, cfg.secret_key.as_bytes(), form)
        .map(|user| HttpResponse::Ok().body(format!("{:?}", user)))
        .unwrap_or_else(|err| HttpResponse::InternalServerError().finish())
}

fn get_register(req: HttpRequest<State>) -> HttpResponse {
    let state = req.state();
    let ctx = Context::new();

    state
        .render("user/register.html", &ctx)
        .map(|content| HttpResponse::Ok().body(content))
        .map(|err| err.into())
        .unwrap_or_else(|err| {
            error!("Error during Tera rendering: {}", err);
            HttpResponse::InternalServerError().finish()
        })
}
