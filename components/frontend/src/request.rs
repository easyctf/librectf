use actix_web::{middleware::session::RequestSession, FromRequest, HttpRequest};
use core::{config::WebConfig as Config, db::Connection, models::User, State};
use failure::Error;
use tera::Context;

use flash::RequestFlash;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub id: i32,
    pub admin: bool,
    pub name: String,
}

impl From<User> for SessionUser {
    fn from(user: User) -> Self {
        SessionUser {
            id: user.id,
            admin: user.admin,
            name: user.name,
        }
    }
}

pub struct Request {
    pub cfg: Config,
    pub ctx: Context,
    pub state: State,
    pub user: Option<SessionUser>,
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

        let user = req
            .session()
            .get::<SessionUser>("user")
            .map_err(|err| format_err!("Error extracting user: {}", err))?
            .map(|user| {
                ctx.insert("user", &user);
                user
            });
        ctx.insert("logged_in", &user.is_some());

        let flashes = req.flashes()?;
        req.session().remove("flashes");
        ctx.insert("flashes", &flashes);

        Ok(Request {
            cfg,
            ctx,
            state,
            user,
        })
    }
}
