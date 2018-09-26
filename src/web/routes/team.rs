use web::{guards::ContextGuard, Template};

#[get("/")]
fn get_index(ctx: ContextGuard) -> Template {
    Template::render("team/index.html", &ctx)
}
