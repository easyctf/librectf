#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde;

mod users;

use rocket_db_pools::{sqlx, Database};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Database)]
#[database("librectf")]
struct Db(sqlx::SqlitePool);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index])
}
