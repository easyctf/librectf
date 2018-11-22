use actix_web::{
    self,
    middleware::{Middleware, Started},
    HttpRequest, HttpResponse,
};
use core::models::{User, Team};
use diesel::prelude::*;

use State;

pub struct TeamRequired;

impl Middleware<State> for TeamRequired {
    fn start(&self, req: &HttpRequest<State>) -> actix_web::Result<Started> {
        let ext = req.extensions();
        let state = req.state();

        let user = match ext.get::<User>() {
            Some(user) => user,
            None => return Ok(Started::Response(HttpResponse::Unauthorized().finish())),
        };

        let team_id = match user.team_id {
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
