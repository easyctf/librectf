use tera::Context;

use super::Template;

#[get("/register")]
fn register() -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}
