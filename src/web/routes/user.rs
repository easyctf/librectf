use bcrypt;
use diesel::{self, prelude::*};
use rocket::request::Form;

use db::Connection;
use models::NewUser;
use web::{ContextGuard, Template};

#[derive(FromForm)]
struct RegisterForm {
    email: String,
    password: String,
}

impl<'a> From<&'a RegisterForm> for NewUser<'a> {
    fn from(form: &'a RegisterForm) -> Self {
        let password = bcrypt::hash(&form.password, bcrypt::DEFAULT_COST).unwrap();
        NewUser {
            email: form.email.as_ref(),
            password: password,
        }
    }
}

#[get("/register")]
fn get_register(ctx: ContextGuard) -> Template {
    Template::render("user/register.html", &ctx)
}

#[post("/register", data = "<form>")]
fn post_register(db: Connection, form: Form<RegisterForm>) {
    use schema::users;
    let new_user: NewUser = form.get().into();
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&*db);
}

#[get("/settings")]
fn get_settings(ctx: ContextGuard) -> Template {
    Template::render("base/index.html", &ctx)
}
