use web::{guards::ContextGuard, Template};

#[get("/")]
fn get_index(ctx: ContextGuard) -> Template {
    Template::render("base/index.html", &ctx)
}
