use core::{
    models::{Team, User},
    teams::CreateForm,
    Error, UserErrorKind,
};
use http::uri::Uri;
use warp::Filter;
use wtforms::Form;

use crate::extractors::{db_conn, get, get_context, navbar, Context};
use crate::guards::require_login;
use crate::render::render_template;

pub fn get_create() -> Resp!() {
    require_login()
        .map(|_| ())
        .untuple_one()
        .and(navbar())
        .and(get_context())
        .and_then(|ctx: Context| render_template("teams/create.html", ctx.into()))
}

pub fn post_create() -> Resp!() {
    require_login()
        .and(warp::body::form().and_then(|form: CreateForm| {
            form.validate()
                .map_err(|_| {
                    Error::user(
                        "bad username or password",
                        UserErrorKind::BadUsernameOrPassword,
                    )
                })
                .map_err(warp::reject::custom)
        }))
        .and(db_conn())
        .and_then(|user: User, form, conn| {
            core::teams::create_team(&conn, user.id, &form).map_err(warp::reject::custom)
        })
        .map(|_team_id: i32| warp::redirect::redirect(Uri::from_static("/users/profile")))
}

pub fn get_index() -> Resp!() {
    let team = get::<Team>().map(|team: Team| {
        let uri = format!("/teams/profile/{}", team.id);
        // TODO: don't unwrap
        warp::redirect::redirect(Uri::from_shared(uri.into()).unwrap())
    });
    let no_team = warp::any().map(|| warp::redirect::redirect(Uri::from_static("/teams/create")));
    navbar().and(team.or(no_team))
}
