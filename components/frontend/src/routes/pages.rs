use actix_web::{HttpRequest, HttpResponse};
use core::{pages::get_page, State};
use tera::Context;

const WELCOME_MESSAGE: &'static str = "Welcome to LibreCTF! You're seeing this message because you haven't set up your index page yet.";

pub fn handler(req: HttpRequest<State>) -> HttpResponse {
    let state = req.state();
    let mut ctx = Context::new();

    // look up the page
    match get_page("") {
        Ok(content) => ctx.insert("content", &content),
        Err(_) => ctx.insert("content", WELCOME_MESSAGE),
    };

    state
        .renderer(|tera| {
            tera.render("index.html", &ctx)
                .map(|content| HttpResponse::Ok().body(content))
                .map(|err| err.into())
                .unwrap_or_else(|err| {
                    error!("Error during Tera rendering: {}", err);
                    HttpResponse::InternalServerError().finish()
                })
        }).unwrap_or_else(|err| {
            error!("{}", err);
            HttpResponse::InternalServerError().finish()
        })
}
