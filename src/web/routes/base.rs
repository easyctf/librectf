use tera::Context;

use super::Template;

#[get("/")]
fn get_index() -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}
