#[macro_use]
extern crate serde_derive;

use core::{DbPool, Error};
use lazy_static::lazy_static;
use packer::Packer;
use tera::Tera;
use warp::{filters::BoxedFilter, http::Response, Filter, Reply};

#[macro_use]
mod macros;

mod base;
mod extractors;
mod render;
mod users;

#[derive(Packer)]
#[folder = "frontend/static"]
struct Assets;

#[derive(Packer)]
#[folder = "frontend/templates"]
struct Templates;

lazy_static! {
    static ref Renderer: Tera = {
        let mut tera = Tera::default();
        tera.add_raw_templates(
            Templates::list()
                .map(|k| (k, Templates::get_str(k).unwrap()))
                .collect::<Vec<_>>(),
        )
        .expect("couldn't load tera templates");
        tera
    };
}

pub fn routes(db: &DbPool) -> BoxedFilter<(impl Reply,)> {
    let routes = route_any! {
        GET ("users" / "register") => users::get_register(),
        POST ("users" / "register") => users::post_register(),
        GET () => base::get_index(),
    }
    .recover(Error::reply);

    let statics = warp::path("static")
        .and(warp::path::param())
        .and_then(|path: String| Assets::get(&path).ok_or_else(warp::reject::not_found))
        .map(|contents| Response::builder().body(contents));
    statics.or(routes).boxed()
}
