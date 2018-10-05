use std::fs::File;
use std::io::Read;
use std::path::Path;

use errors::{FileOpenError, FileReadError};
use Error;

pub fn read_file(path: impl AsRef<Path>) -> Result<String, Error> {
    let mut file = match File::open(path.as_ref()) {
        Ok(file) => file,
        Err(err) => return Err(FileOpenError(err).into()),
    };
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(err) => Err(FileReadError(err).into()),
    }
}
