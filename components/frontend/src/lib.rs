extern crate actix_web;
extern crate core;
extern crate futures;
#[macro_use]
extern crate embed;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tera;

mod flash;
mod request;
mod routes;

use actix_web::App;
use core::State;

use core::Error;
use request::Request;

#[derive(Embed)]
#[folder = "components/frontend/templates"]
struct Templates;

pub fn app(mut state: State) -> Result<App<State>, Error> {
    let templates = Templates::list()
        .filter_map(|name| {
            Templates::get(name)
                .and_then(|contents| String::from_utf8(contents).ok())
                .map(|contents| (String::from(name), contents))
        })
        .collect::<Vec<_>>();
    state.add_templates(
        templates
            .iter()
            .map(|(a, b)| (a.as_ref(), b.as_ref()))
            .collect::<Vec<_>>(),
    )?;
    Ok(routes::router(state))
}
