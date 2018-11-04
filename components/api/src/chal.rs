use actix_web::{App, HttpRequest, HttpResponse};
use core::models::Challenge;
use diesel::RunQueryDsl;

use super::{
    user::{LoginClaims, LoginRequired},
    APIMiddleware, DbConn, State,
};

pub fn app(state: State) -> App<State> {
    App::with_state(state)
        .middleware(APIMiddleware)
        .middleware(LoginRequired)
        .prefix("/chal")
        .resource("/list", |r| r.get().with(list))
        .resource("/submit", |r| r.post().with(submit))
}

fn list((req, db): (HttpRequest<State>, DbConn)) -> HttpResponse {
    use core::schema::chals::dsl::*;

    let ext = req.extensions();
    let claims = ext.get::<LoginClaims>().unwrap();

    chals
        .load::<Challenge>(&*db)
        .map(|list| {
            let list = list
                .iter()
                .map(|chal| {
                    json!({
                        "title": chal.title,
                        "value": chal.value,
                    })
                }).collect::<Vec<_>>();
            HttpResponse::Ok().json(list)
        }).unwrap_or_else(|err| {
            error!("Diesel error while listing chals: {:?}", err);
            HttpResponse::InternalServerError().json(json!(null))
        })
}

fn submit((req, db): (HttpRequest<State>, DbConn)) -> HttpResponse {
    HttpResponse::NotFound().json("d")
}
