use std::fmt;
use std::io;

use failure::Fail;

#[derive(Debug)]
pub struct CustomError(String);

error_wrapper!(AddressBindError: io::Error = "Failed to bind to the address or port");
error_wrapper!(FileOpenError: io::Error = "Failed to access file");
error_wrapper!(FileReadError: io::Error = "Failed to read file");
error_wrapper!(DirTraversalError: io::Error = "Failed to traverse the directory");

error_derive_from!(Error = {
    ::serde_json::Error["Error during the serialization of JSON"] => JSONSerialization,
    DirTraversalError[""] => DirTraversal,
    FileOpenError[""] => FileOpen,
    FileReadError[""] => FileRead,
    AddressBindError[""] => AddressBind,
});

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
