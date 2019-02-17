use core::{DbConn, Error, State};
use std::ops::{Deref, DerefMut};
use tera::Context as TeraContext;
use warp::{Filter, Rejection};

use crate::session::Session;
pub use warp::ext::get;

pub fn db_conn() -> impl Clone + Filter<Extract = (DbConn,), Error = Rejection> {
    get::<State>().and_then(|state: State| state.get_connection().map_err(warp::reject::custom))
}

#[derive(Clone, Default)]
pub struct Context(pub TeraContext);

impl Deref for Context {
    type Target = TeraContext;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Into<TeraContext> for Context {
    fn into(self) -> TeraContext {
        self.0
    }
}

pub fn get_context() -> impl Clone + Filter<Extract = (Context,), Error = Rejection> {
    get::<Context>()
        .or(warp::any().map(|| Context::default()))
        .unify()
}

fn render_navbar(session: &Session, ctx: &mut Context) -> Result<(), Error> {
    // retrieve the user data
    if let Some(user) = session.get_user() {
        ctx.insert("user", &user);
    }

    Ok(())
}

/// Generates all of the information needed to populate the navbar.
pub fn navbar() -> impl Clone + Filter<Extract = (), Error = Rejection> {
    get::<Session>()
        .and(get_context())
        .map(|session: Session, mut ctx: Context| {
            render_navbar(&session, &mut ctx).unwrap_or_else(|err| {
                println!("err: {:?}", err);
            });
            warp::ext::set::<Context>(ctx);
        })
        .untuple_one()
}
