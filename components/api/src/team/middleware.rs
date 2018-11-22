use actix_web::{
    self,
    middleware::{Middleware, Started},
    HttpRequest, HttpResponse,
};
use core::models::{Team, User};
use diesel::prelude::*;

use user::LoginRequired;
use State;

pub struct TeamRequired;

impl Middleware<State> for TeamRequired {
    fn start(&self, req: &HttpRequest<State>) -> actix_web::Result<Started> {
        // first make sure we're logged in
        LoginRequired::start(&LoginRequired, req)?;

        let state = req.state();

        let team_id = {
            let ext = req.extensions();
            match ext.get::<User>() {
                Some(user) => user.team_id,
                None => return Ok(Started::Response(HttpResponse::Unauthorized().finish())),
            }
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
                return Ok(Started::Response(HttpResponse::Unauthorized().finish()));
            }
        };

        let mut ext = req.extensions_mut();
        ext.insert(team);
        Ok(Started::Done)
    }
}
