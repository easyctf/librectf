use std::ops::Deref;

use actix_web::{middleware::session::RequestSession, FromRequest, HttpRequest};
use core::{config::WebConfig as Config, db::Connection, State};
use failure::Error;
use tera::Context;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    current_user: Option<SessionUser>,
}

impl Default for Session {
    fn default() -> Self {
        Session { current_user: None }
    }
}

pub struct Request {
    pub cfg: Config,
    pub session: Session,
    pub ctx: Context,
    pub state: State,
}

impl Request {
    pub fn get_connection(&self) -> Result<Connection, Error> {
        self.state.get_connection()
    }
}

impl FromRequest<State> for Request {
    type Config = ();
    type Result = Result<Self, Error>;

    fn from_request(req: &HttpRequest<State>, _: &Self::Config) -> Self::Result {
        let state = req.state().clone();
        let cfg = state.get_web_config().unwrap().clone();

        let mut ctx = Context::new();
        let session = match req
            .session()
            .get::<Session>("session")
            .map_err(|err| format_err!("Failed to get session data: {}", err))?
        {
            Some(session) => session,
            None => {
                let session = Session::default();
                req.session().set("session", session.clone());
                session
            }
        };
        ctx.insert("logged_in", &session.current_user.is_some());

        Ok(Request {
            cfg,
            ctx,
            session,
            state,
        })
    }
}
