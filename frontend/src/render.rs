use core::{Error, ErrorKind};
use tera::Context;
use warp::Rejection;

use crate::Renderer;

pub fn render_template(path: impl AsRef<str>, ctx: Context) -> Result<String, Rejection> {
    Renderer
        .render(path.as_ref(), ctx)
        .map_err(|_err| Error::new("render error", ErrorKind::Tera))
        .map_err(warp::reject::custom)
}
