use actix_web::{Scope, HttpResponse};
use core::{State, self};

use request::Request;

pub fn scope(app: Scope<State>) -> Scope<State> {
    app.resource("/list", |r| r.get().with(get_list))
}

pub fn get_list(mut req: Request) -> HttpResponse {
    let db = req.state.get_connection().unwrap();

    let chals = match core::chal::list_all(db) {
        Ok(chals) => chals,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    req.ctx.insert("challenges", &chals);

    req.state
        .render("chal/list.html", &req.ctx)
        .map(|content| HttpResponse::Ok().body(content))
        .unwrap_or_else(|err| {
            error!("Error during Tera rendering: {}", err);
            HttpResponse::InternalServerError().finish()
        })
}