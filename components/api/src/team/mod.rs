pub mod manage;
pub mod middleware;

use core::models::{Invitation, NewTeam, Team, User};
use diesel::{prelude::*, result::Error::RollbackTransaction};
use failure::Error;

pub use self::middleware::TeamRequired;
use DbConn;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTeamForm {
    name: String,
}

pub fn create_team(db: DbConn, uid: i32, form: CreateTeamForm) -> Result<(), Error> {
    use diesel::result::Error::RollbackTransaction;

    let new_team = NewTeam {
        name: form.name,
        captain_id: uid,
    };

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
        }

        Ok(())
    }).map_err(|err| err.into())
}

fn get_team_id(db: &DbConn, user_id: i32) -> Result<Option<i32>, Error> {
    use core::schema::users::dsl::{id, users};
    users
        .filter(id.eq(user_id))
        .first::<User>(&**db)
        .map(|user| user.team_id)
        .map_err(|err| err.into())
}

#[derive(Serialize, Deserialize)]
pub struct TeamProfile {
    pub id: i32,
    pub name: String,
    pub banned: bool,
}

fn get_team_profile(db: &DbConn, team_id: i32) -> Result<TeamProfile, Error> {
    use core::schema::teams::dsl::{id, teams};
    teams
        .filter(id.eq(&team_id))
        .first::<Team>(&**db)
        .map(|team| TeamProfile {
            id: team.id,
            name: team.name,
            banned: team.banned,
        }).map_err(|err| err.into())
}

pub fn my_profile(db: DbConn, user_id: i32) -> Result<Option<TeamProfile>, Error> {
    get_team_id(&db, user_id).and_then(|opt| match opt {
        Some(team_id) => get_team_profile(&db, team_id).map(|team| Some(team)),
        None => Ok(None),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AcceptForm {
    pub team_id: i32,
}

pub fn accept_invite(db: DbConn, uid: i32, form: AcceptForm) -> Result<(), Error> {
    use core::schema::invitations::dsl::{invitations, team_id, user_id};

    db.transaction(|| {
        // first make sure the invite exists
        match {
            invitations
                .filter(user_id.eq(uid))
                .filter(team_id.eq(form.team_id))
                .first::<Invitation>(&*db)
        } {
            Ok(_) => (),
            Err(err) => {
                error!("lol {}", err);
                return Err(RollbackTransaction);
            }
        }

        // now delete the invite
        // (this can probably be merged with the previous request as an error handler)
        if let Err(err) = {
            diesel::delete(
                invitations
                    .filter(user_id.eq(uid))
                    .filter(team_id.eq(form.team_id)),
            ).execute(&*db)
        } {
            error!("lol {}", err);
            return Err(RollbackTransaction);
        }

        // now update the user's team
        if let Err(err) = {
            use core::schema::users::dsl::{team_id, users};
            diesel::update(users.find(uid))
                .set(team_id.eq(form.team_id))
                .execute(&*db)
        } {
            error!("lol {}", err);
            return Err(RollbackTransaction);
        }

        Ok(())
    }).map_err(|err| err.into())
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Invite {
    pub team: String,
}

pub fn get_invites(db: DbConn, user_id: i32) -> Result<Vec<Invite>, Error> {
    use core::schema::{invitations, teams};

    invitations::table
        .inner_join(teams::table)
        .select((teams::name,))
        .filter(invitations::user_id.eq(user_id))
        .load::<Invite>(&*db)
        .map_err(|err| err.into())
}
