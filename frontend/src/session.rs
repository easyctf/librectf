use cookie::{Cookie, CookieJar, Key, SameSite};
use core::State;
use serde_derive::{Deserialize, Serialize};
use warp::{Filter, Rejection};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    pub user_id: Option<i32>,
}

pub fn extract() -> impl Clone + Filter<Extract = (), Error = Rejection> {
    warp::any()
        .map(|| {
            warp::ext::set::<Session>(Session::default());
        })
        .untuple_one()
        .or(warp::ext::get::<State>()
            .and(warp::filters::header::header::<String>("cookie"))
            .map(move |state: State, data: String| {
                let mut jar = CookieJar::new();
                let cookies = data.split(";");
                for cookie in cookies {
                    if let Ok(cookie) = Cookie::parse_encoded(cookie) {
                        jar.add(cookie.into_owned());
                    }
                }

                let key = Key::from_master(state.get_secret_key().as_bytes());
                warp::ext::set::<Session>(
                    jar.private(&key)
                        .get("session")
                        .and_then(|data| serde_urlencoded::from_str(data.value()).ok())
                        .unwrap_or_else(|| Session::default()),
                );
            })
            .untuple_one())
        .unify()
}

pub fn apply() -> impl Clone + Filter<Extract = (Option<String>,), Error = Rejection> {
    warp::ext::get::<State>()
        .and(warp::ext::get::<Session>())
        .and_then(|state: State, session: Session| {
            println!("session is {:?}", session);
            let key = Key::from_master(state.get_secret_key().as_bytes());
            let mut jar = CookieJar::new();
            serde_urlencoded::to_string(session)
                .map(|data| {
                    jar.private(&key).add(
                        Cookie::build("session", data)
                            .secure(true)
                            .http_only(true)
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
