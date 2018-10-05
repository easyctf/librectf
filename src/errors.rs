use std::fmt;
use std::io;

use failure::Fail;
use rocket;
use serde_json;

#[derive(Debug, Fail)]
#[fail(display = "Error traversing the directory")]
pub struct DirError(#[cause] pub io::Error);

#[derive(Debug)]
pub struct CustomError(String);

macro_rules! error_derive_from {
    ([$($error:path[$v:expr] => $into:ident,)*]) => {
        #[derive(Debug, Fail)]
        pub enum Error {
            #[fail(display = "")]
            Custom(#[cause] CustomError),
            $(
                #[fail(display = $v)]
                $into(#[cause] $error),
            )*
        }

        $(
            impl From<$error> for Error {
                fn from(err: $error) -> Self {
                    Error::$into(err)
                }
            }
        )*
    };
}

error_derive_from!([
    rocket::error::LaunchError["Error launching Rocket application"] => RocketLaunch,
    serde_json::Error["Error during the serialization of JSON"] => JSONSerialization,
    DirError["Error traversing the directory"] => Dir,
]);

impl CustomError {
    pub fn new(s: impl AsRef<str>) -> Self {
        CustomError(s.as_ref().to_owned())
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Fail for CustomError {}

impl From<CustomError> for Error {
    fn from(err: CustomError) -> Self {
        Error::Custom(err)
    }
}
