use actix_web::{middleware::session::RequestSession, Form, HttpRequest, HttpResponse, Scope};
use core::{
    user::auth::{login_user, register_user, LoginForm, RegisterForm},
    State,
};

use request::{Request, SessionUser};

pub fn scope(app: Scope<State>) -> Scope<State> {
    app.resource("/login", |r| {
        r.get().with(get_login);
        r.post().with(post_login)
    }).resource("/register", |r| {
        r.get().with(get_register);
        r.post().with(post_register)
    }).resource("/logout", |r| r.get().with(get_logout))
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

fn post_login((r, req, form): (HttpRequest<State>, Request, Form<LoginForm>)) -> HttpResponse {
    let db = req.state.get_connection().unwrap();
    let form = form.into_inner();

    login_user(db, form)
        .map(|user| {
            let s_user = SessionUser {
                id: user.id,
                name: user.name.clone(),
            };
            r.session().set("user", s_user);
            HttpResponse::SeeOther().header("Location", "/").finish()
        }).unwrap_or_else(|err| HttpResponse::InternalServerError().finish())
}

fn get_logout(req: HttpRequest<State>) -> HttpResponse {
    req.session().clear();
    HttpResponse::SeeOther().header("Location", "/").finish()
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

fn post_register((req, form): (Request, Form<RegisterForm>)) -> HttpResponse {
    let db = req.state.get_connection().unwrap();
    let form = form.into_inner();

    register_user(db, form)
        .map(|user| HttpResponse::Ok().body(format!("{:?}", user)))
        .unwrap_or_else(|err| HttpResponse::InternalServerError().finish())
}
