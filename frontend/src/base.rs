use warp::Filter;

use crate::extractors::{get_context, navbar, Context};
use crate::render::render_template;

pub fn get_index() -> Resp!() {
    warp::path::end()
        .and(navbar())
        .and(get_context())
        .and_then(|ctx: Context| render_template("base/index.html", ctx.into()))
}
