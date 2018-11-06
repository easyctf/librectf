use core::models::{NewTeam, Team};
use diesel::{self, prelude::*};
use failure::Error;

use DbConn;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamForm {
    name: String,
}

pub fn create_team(db: DbConn, uid: i32, form: CreateTeamForm) -> Result<(), Error> {
    use diesel::result::Error::RollbackTransaction;
    let new_team = NewTeam { name: form.name };
    db.transaction(|| {
        // first, create the actual team
        match {
            use core::schema::teams;
            diesel::insert_into(teams::table)
                .values(new_team)
                .execute(&*db)
        } {
            Err(err) => {
                error!("Diesel error on team/create (1): {}", err);
                return Err(RollbackTransaction);
            }
            _ => (),
        };

        // now get the team name
        let new_team = match {
            use core::schema::teams::dsl::*;
            teams.order_by(id.desc()).first::<Team>(&*db)
        } {
            Ok(team) => team,
            Err(err) => {
                error!("Diesel error on team/create (2): {}", err);
                return Err(RollbackTransaction);
            }
        };

        // now update the users
        if let Err(err) = {
            use core::schema::users::dsl::*;
            diesel::update(users.find(uid))
                .set(team_id.eq(new_team.id))
                .execute(&*db)
        } {
            error!("Diesel error on team/create (3): {}", err);
            return Err(RollbackTransaction);
        };

        Ok(())
    }).map_err(|err| err.into())
}

// fn get_team_profile(team_id: i32, db: DbConn) -> Option<impl Serialize> {
//     use core::schema::teams::dsl::*;
//     teams
//         .filter(id.eq(&team_id))
//         .first::<Team>(&*db)
//         .map(|team| {
//             json!({
//                 "team": {
//                     "id": team.id,
//                     "name": team.name,
//                     "banned": team.banned,
//                 }
//             })
//         }).ok()
// }

// fn me((req, db): (HttpRequest<State>, DbConn)) -> HttpResponse {
//     // TODO: don't unwrap
//     let ext = req.extensions();
//     error!("{:?}", ext);
//     let team_id = match ext.get::<LoginClaims>() {
//         Some(claims) => {
//             let user = {
//                 use core::schema::users::dsl::*;
//                 users.filter(id.eq(claims.id)).first::<User>(&*db).unwrap()
//             };
//             error!("{:?}", user);
//             match user.team_id {
//                 Some(team_id) => team_id,
//                 // TODO: think of a better status code for this
//                 None => return HttpResponse::Ok().json(json!({ "team": null })),
//             }
//         }
//         None => return HttpResponse::NotFound().json(json!(null)),
//     };

//     match get_team_profile(team_id, db) {
//         Some(team) => HttpResponse::Ok().json(team),
//         None => HttpResponse::NotFound().json(json!(null)),
//     }
// }

// fn profile((path, db): (Path<(i32,)>, DbConn)) -> HttpResponse {
//     match get_team_profile(path.0, db) {
//         Some(team) => HttpResponse::Ok().json(team),
//         None => HttpResponse::NotFound().json(json!(null)),
//     }
// }
