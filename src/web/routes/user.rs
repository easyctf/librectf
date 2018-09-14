use tera::Context;

use web::{Template, UserGuard};

#[get("/register")]
fn register() -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}

#[get("/settings")]
fn settings(_user: UserGuard) -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}
