use actix_web::{HttpRequest, HttpResponse};
use core::{pages::get_page, State};

use Request;

const WELCOME_MESSAGE: &str = "Welcome to LibreCTF! You're seeing this message because your admin probably haven't set up the index page yet.";

#[derive(Embed)]
#[folder = "components/frontend/static"]
struct Static;

pub fn handler(req: Request) -> HttpResponse {
    let Request { state, mut ctx, .. } = req;

    // TODO: look up the page
    match get_page("") {
        Ok(content) => ctx.insert("content", &content),
        Err(_) => ctx.insert("content", WELCOME_MESSAGE),
    };

    state
        .render("page.html", &ctx)
        .map(|content| {
            HttpResponse::Ok()
                .header("Content-Type", "text/html")
                .body(content)
        })
        .unwrap_or_else(|err| {
            error!("Error during Tera rendering: {}", err);
            HttpResponse::InternalServerError().finish()
        })
}

pub fn statics(req: HttpRequest<State>) -> HttpResponse {
    let path = req.match_info().query::<String>("path").unwrap();
    match Static::get(&path) {
        Some(contents) => HttpResponse::Ok().body(contents),
        None => HttpResponse::NotFound().finish(),
    }
}
