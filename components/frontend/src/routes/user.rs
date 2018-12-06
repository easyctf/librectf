use actix_web::{HttpRequest, HttpResponse, Scope};
use core::State;
use tera::Context;

pub fn scope(app: Scope<State>) -> Scope<State> {
    app.resource("/login", |r| r.with(login))
}

fn login(req: HttpRequest<State>) -> HttpResponse {
    let state = req.state();
    let ctx = Context::new();

    state.render("user/login.html", &ctx)
        .map(|content| HttpResponse::Ok().body(content))
        .map(|err| err.into())
        .unwrap_or_else(|err| {
            error!("Error during Tera rendering: {}", err);
            HttpResponse::InternalServerError().finish()
        })
}
