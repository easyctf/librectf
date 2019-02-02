use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;

use warp::{Rejection, Reply};

#[derive(Clone, Debug)]
pub enum UserError {}

#[derive(Clone, Debug)]
pub enum Error {
    R2d2(Arc<::r2d2::Error>),
    Tera(Arc<::tera::ErrorKind>),
    User(UserError),
    // DEBUG
    Unit,
}

impl Error {
    pub fn reply(err: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(_err) = &err.find_cause::<Error>() {
            Ok("hello")
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
