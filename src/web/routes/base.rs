use tera::Context;

use super::Template;

#[get("/")]
fn index() -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}
