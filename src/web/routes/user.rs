use orm::{ConnectionPool, Model};
use rocket::State;
use tera::Context;

use models::User;
use web::{Template, UserGuard};

#[get("/register")]
fn register() -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}

#[get("/settings")]
fn settings(db: State<ConnectionPool>, _user: UserGuard) -> Template {
    let ctx = Context::new();

    // testing out the model
    let query = User::query();
    db.run(query);

    Template::render("base/index.html", &ctx)
}
