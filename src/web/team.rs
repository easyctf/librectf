use actix_web::{App, HttpRequest, HttpResponse, Json};

use super::{user::LoginRequired, DbConn, State};

pub fn app(state: State) -> App<State> {
    App::with_state(state)
        .middleware(LoginRequired)
        .prefix("/team")
        .resource("/create", |r| r.post().with(create))
        .resource("/profile", |r| r.post().with(profile))
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTeamForm {
    name: String,
}

fn create(_form: Json<CreateTeamForm>) -> HttpResponse {
    HttpResponse::Ok().json("lol")
}

fn profile(db: DbConn) -> HttpResponse {
    HttpResponse::Ok().json("")
}
