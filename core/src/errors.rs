use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

use warp::{Rejection, Reply};

#[derive(Clone, Debug)]
pub enum UserError {
    BadUsernameOrPassword,
}

#[derive(Clone, Debug)]
pub enum Error {
    Bcrypt(Arc<::bcrypt::BcryptError>),
    Diesel(Arc<::diesel::result::Error>),
    Migrations(Arc<::diesel_migrations::RunMigrationsError>),
    R2d2(Arc<::r2d2::Error>),
    Tera(Arc<::tera::ErrorKind>),
    User(UserError),
    // DEBUG
    Unit,
}

impl Error {
    pub fn reply(err: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(err) = &err.find_cause::<Error>() {
            Ok(format!("there's an error: {:?}", err))
        } else {
            Err(err)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        // TODO: do this
        Ok(())
    }
}

impl StdError for Error {}

impl From<::bcrypt::BcryptError> for Error {
    fn from(err: ::bcrypt::BcryptError) -> Self {
        Error::Bcrypt(Arc::new(err))
    }
}

impl From<::diesel::result::Error> for Error {
    fn from(err: ::diesel::result::Error) -> Self {
        Error::Diesel(Arc::new(err))
    }
}

impl From<::diesel_migrations::RunMigrationsError> for Error {
    fn from(err: ::diesel_migrations::RunMigrationsError) -> Self {
        Error::Migrations(Arc::new(err))
    }
}

impl From<::r2d2::Error> for Error {
    fn from(err: ::r2d2::Error) -> Self {
        Error::R2d2(Arc::new(err))
    }
}

impl From<::tera::Error> for Error {
    fn from(err: ::tera::Error) -> Self {
        Error::Tera(Arc::new(err.kind))
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::Unit
    }
}
