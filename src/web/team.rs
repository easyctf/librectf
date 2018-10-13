use actix_web::{App, HttpResponse, Json};

use super::{user::LoginMiddleware, State};

pub fn app(state: State) -> App<State> {
    App::with_state(state)
        .middleware(LoginMiddleware)
        .resource("/team/create", |r| r.post().with(create))
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTeamForm {
    name: String,
}

fn create(_form: Json<CreateTeamForm>) -> HttpResponse {
    HttpResponse::Ok().json("lol")
}
