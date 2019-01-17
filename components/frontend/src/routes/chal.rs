use actix_web::Scope;
use core::{self, Error, State};

use request::Request;

pub fn scope(app: Scope<State>) -> Scope<State> {
    app.resource("/list", |r| r.get().with(get_list))
}

pub fn get_list(mut req: Request) -> Result<String, Error> {
    let db = req.state.get_connection().unwrap();

    let chals = core::chal::list_all(db)?;
    req.ctx.insert("challenges", &chals);

    req.state
        .render("chal/list.html", &req.ctx)
        .map_err(|err| err.into())
}
