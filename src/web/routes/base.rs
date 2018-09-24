use super::Template;
use web::guards::ContextGuard;

#[get("/")]
fn get_index(ctx: ContextGuard) -> Template {
    Template::render("base/index.html", &ctx)
}
