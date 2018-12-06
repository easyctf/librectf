use std::path::PathBuf;

use actix_web::{HttpRequest, HttpResponse};
use core::{pages::get_page, State};
use tera::Context;

const WELCOME_MESSAGE: &'static str = "Welcome to LibreCTF! You're seeing this message because you probably haven't set up your index page yet. Head over to the admin panel to set it up!";

#[derive(Embed)]
#[folder = "components/frontend/static"]
struct Static;

pub fn handler(req: HttpRequest<State>) -> HttpResponse {
    let state = req.state();
    let mut ctx = Context::new();

    // TODO: look up the page
    match get_page("") {
        Ok(content) => ctx.insert("content", &content),
        Err(_) => ctx.insert("content", WELCOME_MESSAGE),
    };

    state
        .render("page.html", &ctx)
        .map(|content| HttpResponse::Ok().body(content))
        .map(|err| err.into())
        .unwrap_or_else(|err| {
            error!("Error during Tera rendering: {}", err);
            HttpResponse::InternalServerError().finish()
        })
}

pub fn statics(req: HttpRequest<State>) -> HttpResponse {
    let path = req.match_info().query::<String>("path").unwrap();
    println!("{:?}", path);
    match Static::get(&path) {
        Some(contents) => HttpResponse::Ok().body(contents),
        None => HttpResponse::NotFound().finish(),
    }
}
