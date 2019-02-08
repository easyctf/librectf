use core::{Error, State};
use std::ops::{Deref, DerefMut};
use tera::Context as TeraContext;
use warp::{Filter, Rejection};

use crate::session::Session;

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
    warp::ext::get::<Context>()
        .or(warp::any().map(|| Context::default()))
        .unify()
}

fn render_navbar(state: &State, session: &Session, ctx: &mut Context) -> Result<(), Error> {
    let conn = state.get_connection()?;

    // retrieve the user data
    if let Some(user_id) = session.user_id {
        conn.fetch_user_id(user_id)
            .map(|user| ctx.insert("user", &user))?;
    }

    Ok(())
}

/// Generates all of the information needed to populate the navbar.
pub fn navbar() -> impl Clone + Filter<Extract = (), Error = Rejection> {
    warp::ext::get::<State>()
        .and(warp::ext::get::<Session>())
        .and(get_context())
        .map(|state: State, session: Session, mut ctx: Context| {
            render_navbar(&state, &session, &mut ctx).unwrap_or_else(|err| {
                println!("err: {:?}", err);
            });
            warp::ext::set::<Context>(ctx);
        })
        .untuple_one()
}
