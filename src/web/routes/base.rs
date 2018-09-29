use web::{guards::ContextGuard, Template};

use db::Connection;

#[get("/")]
fn get_index(ctx: ContextGuard) -> Template {
    Template::render("base/index.html", &ctx)
}

#[get("/scoreboard")]
fn get_scoreboard(db: Connection, ctx: ContextGuard) -> Template {
    use schema::teams;
    
    Template::render("base/index.html", &ctx)
}
