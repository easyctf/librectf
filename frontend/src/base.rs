use tera::Context;
use warp::Filter;

use crate::render::render_template;

pub fn get_index() -> Resp!() {
    warp::any().and_then(|| render_template("base/index.html", Context::new()))
}
