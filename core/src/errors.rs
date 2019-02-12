use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;

use backtrace::Backtrace;
use warp::{Rejection, Reply};

/// ErrorExt trait, based on the avocado lib.
pub trait ErrorExt: StdError {
    /// Similar to `std::error::Error::source()`, but with richer type info.
    fn reason(&self) -> Option<&(dyn ErrorExt + 'static)> {
        None
    }

    /// Returns the deepest possible backtrace, if any.
    fn backtrace(&self) -> Option<&Backtrace> {
        None
    }

    /// Structured error kind.
    fn kind(&self) -> ErrorKind;

    /// Until subtrait coercions are implemented, this helper method
    /// should return the receiver as an `&std::error::Error` trait object.
    fn as_std_error(&self) -> &(dyn StdError + 'static);
}

/// An error caused by the user.
#[derive(Copy, Clone, Debug)]
pub enum UserErrorKind {
    /// The user supplied bad credentials during login.
    BadUsernameOrPassword,
}

/// An error.
#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    #[doc(hidden)]
    Bcrypt,
    #[doc(hidden)]
    Diesel,
    #[doc(hidden)]
    Migrations,
    #[doc(hidden)]
    R2d2,
    #[doc(hidden)]
    Tera,

    /// An error caused by the user.
    ///
    /// During rendering, other errors will result in a 500 page, while user
    /// errors will be propagated to the generated page and presented to the
    /// user.
    User(UserErrorKind),
}

/// The main error trait of the crate.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: Cow<'static, str>,
    backtrace: Option<Backtrace>,
    cause: Option<Box<dyn ErrorExt + Send + Sync + 'static>>,
}

impl Error {
    /// Converts this error into a response. (TODO: generate flashes and a redirect)
    pub fn reply(err: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(err) = &err.find_cause::<Error>() {
            Ok(format!("there's an error: {:?}", err))
        } else {
            Err(err)
        }
    }

    /// Create an error that was caused by a user
    pub fn user<M>(message: M, kind: UserErrorKind) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Error {
            kind: ErrorKind::User(kind),
            message: message.into(),
            backtrace: None,
            cause: None,
        }
    }

    /// Create a message from just a message and a kind
    pub fn new<M>(message: M, kind: ErrorKind) -> Self
    where
        M: Into<Cow<'static, str>>,
    {
        Error {
            message: message.into(),
            kind,
            backtrace: None,
            cause: None,
        }
    }

    /// Chains an error with a cause
    pub fn with_cause<M, E>(message: M, cause: E) -> Self
    where
        M: Into<Cow<'static, str>>,
        E: ErrorExt + Send + Sync + 'static,
    {
        let kind = cause.kind();
        let message = message.into();
        let backtrace = Some(match cause.backtrace() {
            Some(backtrace) => backtrace.clone(),
            None => Backtrace::new(),
        });
        let cause: Option<Box<dyn ErrorExt + Send + Sync + 'static>> = Some(Box::new(cause));
        Error {
            kind,
            message,
            backtrace,
            cause,
        }
    }
}

impl ErrorExt for Error {
    fn kind(&self) -> ErrorKind {
        self.kind
    }
    fn as_std_error(&self) -> &(dyn StdError + 'static) {
        self
    }
}

impl fmt::Display for Error {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        // TODO: do this
        Ok(())
    }
}

impl StdError for Error {}

/// Implementing `ErrorExt` and `From` boilerplate.
macro_rules! impl_error_type {
    ($ty:path, $kind:ident, $message:expr) => {
        impl From<$ty> for Error {
            fn from(error: $ty) -> Self {
                Self::with_cause($message, error)
            }
        }

        impl ErrorExt for $ty {
            fn kind(&self) -> ErrorKind {
                ErrorKind::$kind
            }

            fn as_std_error(&self) -> &(dyn StdError + 'static) {
                self
            }
        }
    };
}

impl_error_type!(::bcrypt::BcryptError, Bcrypt, "bcrypt error");
impl_error_type!(::diesel::result::Error, Diesel, "diesel error");
impl_error_type!(
    ::diesel_migrations::RunMigrationsError,
    Migrations,
    "migrations error"
);
impl_error_type!(::r2d2::Error, R2d2, "r2d2 error");

impl ErrorExt for ::tera::Error {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Tera
    }

    fn as_std_error(&self) -> &(dyn StdError + 'static) {
        self
    }
}
