use cookie::{Cookie, CookieJar, Key, SameSite};
use core::{
    models::{Team, User},
    DbConn, Error, State,
};
use serde_derive::{Deserialize, Serialize};
use warp::{Filter, Rejection};

use crate::extractors::get;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SessionRepr {
    pub user_id: Option<i32>,
    pub team_id: Option<i32>,
}

#[derive(Clone, Default)]
pub struct Session {
    pub user: Option<User>,
    pub team: Option<Team>,
}

impl Session {
    pub fn try_from(conn: &DbConn, repr: SessionRepr) -> Result<Session, Error> {
        let user = repr.user_id.and_then(|id| conn.fetch_user_id(id).ok());
        let team = user
            .as_ref()
            .and_then(|user| user.team_id)
            .and_then(|id| conn.fetch_team_id(id).ok());
        Ok(Session { user, team })
    }
    pub fn repr(self) -> SessionRepr {
        SessionRepr {
            user_id: self.user.map(|user| user.id),
            team_id: self.team.map(|team| team.id),
        }
    }
    pub fn is_empty(&self) -> bool {
        match (&self.user, &self.team) {
            (None, None) => true,
            _ => false,
        }
    }
}

pub fn extract() -> impl Clone + Filter<Extract = (), Error = Rejection> {
    get::<State>()
        .and(warp::filters::header::header::<String>("cookie"))
        .map(move |state: State, data: String| {
            let mut jar = CookieJar::new();
            let cookies = data.split(";");
            for cookie in cookies {
                if let Ok(cookie) = Cookie::parse_encoded(cookie) {
                    jar.add(cookie.into_owned());
                }
            }

            // TODO: don't unwrap
            let conn = state.get_connection().unwrap();
            let key = Key::from_master(state.get_secret_key().as_bytes());
            warp::ext::set::<Session>(
                jar.private(&key)
                    .get("session")
                    .and_then(|data| serde_urlencoded::from_str::<SessionRepr>(data.value()).ok())
                    .and_then(|repr| Session::try_from(&conn, repr).ok())
                    .unwrap_or_else(|| Session::default()),
            );
        })
        .untuple_one()
        .or(warp::any()
            .map(|| {
                warp::ext::set::<Session>(Session::default());
            })
            .untuple_one())
        .unify()
}

pub fn apply() -> impl Clone + Filter<Extract = (Option<String>,), Error = Rejection> {
    get::<State>()
        .and(get::<Session>())
        .and_then(|state: State, session: Session| {
            let mut jar = CookieJar::new();
            let key = Key::from_master(state.get_secret_key().as_bytes());
            serde_urlencoded::to_string(session.repr())
                .map(|data| {
                    jar.private(&key).add(
                        Cookie::build("session", data)
                            // .secure(true) // TODO: enable this based on a config
                            .http_only(true)
                            .path("/")
                            .same_site(SameSite::Strict)
                            .finish(),
                    );
                    Some(format!("{}", jar.get("session").unwrap()))
                })
                .map_err(|err| {
                    println!("err: {:?}", err);
                    warp::reject::custom(err)
                })
        })
        .or(warp::any().map(|| None))
        .unify()
}
