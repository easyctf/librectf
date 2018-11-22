mod middleware;

use core::models::{NewTeam, Team, User};
use diesel::{self, prelude::*};
use failure::Error;

pub use self::middleware::TeamRequired;
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

fn get_team_id(db: &DbConn, user_id: i32) -> Result<Option<i32>, Error> {
    use core::schema::users::dsl::*;
    users
        .filter(id.eq(user_id))
        .first::<User>(&**db)
        .map(|user| user.team_id)
        .map_err(|err| err.into())
}

pub fn me(db: DbConn, user_id: i32) -> Result<Option<TeamProfile>, Error> {
    get_team_id(&db, user_id).and_then(|opt| match opt {
        Some(team_id) => get_team_profile(&db, team_id).map(|team| Some(team)),
        None => Ok(None),
    })
}

#[derive(Serialize, Deserialize)]
pub struct TeamProfile {
    pub id: i32,
    pub name: String,
    pub banned: bool,
}

fn get_team_profile(db: &DbConn, team_id: i32) -> Result<TeamProfile, Error> {
    use core::schema::teams::dsl::*;
    teams
        .filter(id.eq(&team_id))
        .first::<Team>(&**db)
        .map(|team| TeamProfile {
            id: team.id,
            name: team.name,
            banned: team.banned,
        }).map_err(|err| err.into())
}
