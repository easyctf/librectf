use actix_web::ResponseError;

error_wrapper!(DbConnectionError: ::r2d2::Error = "Failed to get database connection from pool.");

error_derive_from!(WebError = {
    DbConnectionError[""] => DbConnection,
    ::bcrypt::BcryptError["Bcrypt error"] => Bcrypt,
});

impl ResponseError for WebError {}
