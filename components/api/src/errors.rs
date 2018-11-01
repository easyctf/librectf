use std::io;

use actix_web::ResponseError;

error_wrapper!(AddressBindError: io::Error = "Failed to bind to the address or port");
error_wrapper!(DbConnectionError: ::r2d2::Error = "Failed to get database connection from pool.");

error_derive_from!(WebError = {
    AddressBindError[""] => AddressBind,
    DbConnectionError[""] => DbConnection,
    ::std::net::AddrParseError["Failed to parse ipv4 host from string."] => Ipv4Parse,
    ::bcrypt::BcryptError["Bcrypt error"] => Bcrypt,
});

impl ResponseError for WebError {}
