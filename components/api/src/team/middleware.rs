use actix_web::{
    self,
    middleware::{Middleware, Started},
    HttpRequest, HttpResponse,
};
use core::models::{Team, User};
use diesel::prelude::*;

use user::LoginRequired;
use State;

pub enum Boolean {
    True,
    False,
}

impl Default for Boolean {
    fn default() -> Boolean {
        Boolean::False
    }
}

/// C is whether or not it's required that the user is the team captain.
#[derive(Default)]
pub struct TeamRequired<C>(pub C);

impl Middleware<State> for TeamRequired<Boolean> {
    fn start(&self, req: &HttpRequest<State>) -> actix_web::Result<Started> {
        // first make sure we're logged in
        LoginRequired::start(&LoginRequired, req)?;

        let state = req.state();

        let (user_id, team_id) = {
            let ext = req.extensions();
            let user = ext.get::<User>().expect("we should be logged in by now");
            (user.id, user.team_id)
        };

        let team_id = match team_id {
            Some(id) => id,
            None => return Ok(Started::Response(HttpResponse::Unauthorized().finish())),
        };

        let db = state.get_connection()?;
        let team = match {
            use core::schema::teams::dsl::{id, teams};
            teams.filter(id.eq(team_id)).first::<Team>(&*db)
        } {
            Ok(team) => team,
            Err(err) => {
                error!("Error loading team from database: {:?}", err);
                return Ok(Started::Response(HttpResponse::Unauthorized().json(
                    json!({
                        "error": "no_team::",
                    }),
                )));
            }
        };

        if let Boolean::True = self.0 {
            if user_id != team.captain_id {
                return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
            }
        }

        let mut ext = req.extensions_mut();
        ext.insert(team);
        Ok(Started::Done)
    }
}
