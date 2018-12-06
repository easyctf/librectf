extern crate actix_web;
extern crate core;
#[macro_use]
extern crate embed;
#[macro_use]
extern crate log;
extern crate tera;

mod routes;

use actix_web::App;
use core::State;

#[derive(Embed)]
#[folder = "components/frontend/templates"]
struct Templates;

pub fn app(mut state: State) -> App<State> {
    let templates = Templates::list().filter_map(|name| {
        Templates::get(name)
            .and_then(|contents| String::from_utf8(contents).ok())
            .map(|contents| (name, contents))
    });
    state.add_templates(templates);
    routes::router(state)
}
