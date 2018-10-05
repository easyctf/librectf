use serde_json;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error during serialization of JSON")]
    JSONSerialization(#[cause] serde_json::Error),
}

macro_rules! error_derive_from {
    ($error:path => $into:ident) => {
        impl From<$error> for Error {
            fn from(err: $error) -> Self {
                Error::$into(err)
            }
        }
    };
}

error_derive_from!(serde_json::Error => JSONSerialization);
