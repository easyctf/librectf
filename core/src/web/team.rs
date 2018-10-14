use actix_web::{App, HttpRequest, HttpResponse, Json};
use diesel::prelude::*;

use super::{
    user::{LoginClaim, LoginRequired},
    DbConn, State,
};
use models::{Team, User};

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

fn profile((req, db): (HttpRequest<State>, DbConn)) -> HttpResponse {
    // TODO: don't unwrap
    let ext = req.extensions();
    let claims = ext.get::<LoginClaim>().unwrap();

    let user = {
        use schema::users::dsl::*;
        users.filter(id.eq(&claims.id)).first::<User>(&*db).unwrap()
    };
    let team_id = match user.team_id {
        Some(team_id) => team_id,
        // TODO: think of a better status code for this
        None => return HttpResponse::BadRequest().json(json!({ "team": null })),
    };

    let team = {
        use schema::teams::dsl::*;
        teams.filter(id.eq(&team_id)).first::<Team>(&*db).unwrap()
    };

    HttpResponse::Ok().json(json!({
        "team": {
            "id": team.id,
            "name": team.name,
            "banned": team.banned,
        }
    }))
}
