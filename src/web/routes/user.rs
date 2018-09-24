use rocket::State;
use tera::Context;

use web::{Template, UserGuard};

#[derive(FromForm)]
struct RegisterForm {}

#[get("/register")]
fn get_register() -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}

#[post("/register")]
fn post_register() {}

#[get("/settings")]
fn get_settings(_user: UserGuard) -> Template {
    let ctx = Context::new();

    // testing out the model

    Template::render("base/index.html", &ctx)
}
