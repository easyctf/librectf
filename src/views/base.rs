use std::collections::HashMap;

use actix_web::{error, Error, HttpResponse, Query, State};
use tera::Context;

use AppState;

pub fn index(
    (state, query): (State<AppState>, Query<HashMap<String, String>>),
) -> Result<HttpResponse, Error> {
    let s = state
        .templates
        .render("templates/index.html", &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
