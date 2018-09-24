use diesel::prelude::*;
use rocket::request::Form;
use tera::Context;

use web::{Template, UserGuard};
use db::Connection;
use models::NewUser;
use security;

#[derive(FromForm)]
struct RegisterForm {
    email: String,
    password: String,
}

impl<'a> From<&'a RegisterForm> for NewUser<'a> {
    fn from(form: &'a RegisterForm) -> Self {
        NewUser {
            email: form.email.as_ref(),
            password: security::generate_password(&form.password),
        }
    }
}

#[get("/register")]
fn get_register() -> Template {
    let ctx = Context::new();
    Template::render("base/index.html", &ctx)
}

#[post("/register", data = "<form>")]
fn post_register(db: Connection, form: Form<RegisterForm>) {
    use schema::users;
    let new_user: NewUser = form.get().into();
    diesel::insert_into(users::table).values(&new_user).execute(&*db);
}

#[get("/settings")]
fn get_settings(_user: UserGuard) -> Template {
    let ctx = Context::new();

    // testing out the model

    Template::render("base/index.html", &ctx)
}
