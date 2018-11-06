pub mod auth;
mod middleware;

use actix_web::HttpResponse;

use super::DbConn;

pub use self::auth::LoginClaims;
pub use self::middleware::LoginRequired;

fn get_settings(db: DbConn) -> HttpResponse {
    HttpResponse::Ok().json("")
}

fn post_settings(db: DbConn) -> HttpResponse {
    HttpResponse::Ok().json("")
}
