use orm::ConnectionPool;
use rocket::State;
use tera::Context;

use models::User;
use web::{Template, UserGuard};

#[get("/register")]
fn get_register() -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}

#[post("/register")]
fn post_register() {}

#[get("/settings")]
fn get_settings(db: State<ConnectionPool>, _user: UserGuard) -> Template {
    let ctx = Context::new();

    // testing out the model
    let query = db.query((User::model(), User::id()));

    Template::render("base/index.html", &ctx)
}
