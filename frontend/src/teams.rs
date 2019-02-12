use core::models::Team;
use http::uri::Uri;
use warp::Filter;

use crate::extractors::{get, get_context, navbar, Context};
use crate::guards::require_login;
use crate::render::render_template;

pub fn get_create() -> Resp!() {
    require_login()
        .and(navbar())
        .and(get_context())
        .and_then(|ctx: Context| render_template("users/login.html", ctx.into()))
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
