use std::fmt;

use actix_web::ResponseError;

#[derive(Debug)]
pub enum Error {
    PoisonedMutex,
    Custom(String),

    Io(::std::io::Error),

    Actix(::actix_web::Error),
    Bcrypt(::bcrypt::BcryptError),
    Config(::cfg::ConfigError),
    Diesel(::diesel::result::Error),
    Hyper(::hyper::Error),
    R2d2(::r2d2::Error),
    Tera(::tera::ErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::PoisonedMutex => write!(f, "Poisoned mutex"),
            Error::Custom(err) => write!(f, "Error: {}", err),

            Error::Io(err) => write!(f, "IO error: {}", err),

            Error::Actix(err) => write!(f, "Actix error: {}", err),
            Error::Bcrypt(err) => write!(f, "Bcrypt error: {}", err),
            Error::Config(err) => write!(f, "Config error: {}", err),
            Error::Diesel(err) => write!(f, "Diesel error: {}", err),
            Error::Hyper(err) => write!(f, "Hyper error: {}", err),
            Error::R2d2(err) => write!(f, "r2d2 error: {}", err),
            Error::Tera(err) => write!(f, "Tera error: {:?}", err),
        }
    }
}

impl ::std::error::Error for Error {}

impl ResponseError for Error {}

macro_rules! from_impl {
    ($from:path => $into:path) => {
        impl From<$from> for Error {
            fn from(err: $from) -> Self {
                $into(err)
            }
        }
    };
}

impl<T> From<::std::sync::PoisonError<T>> for Error {
    fn from(_: ::std::sync::PoisonError<T>) -> Self {
        Error::PoisonedMutex
    }
}

impl From<::tera::Error> for Error {
    fn from(err: ::tera::Error) -> Self {
        Error::Tera(err.kind)
    }
}

from_impl!(::std::io::Error => Error::Io);

from_impl!(::actix_web::Error => Error::Actix);
from_impl!(::bcrypt::BcryptError => Error::Bcrypt);
from_impl!(::cfg::ConfigError => Error::Config);
from_impl!(::diesel::result::Error => Error::Diesel);
from_impl!(::hyper::Error => Error::Hyper);
from_impl!(::r2d2::Error => Error::R2d2);
