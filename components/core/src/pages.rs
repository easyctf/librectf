use failure::Error;

pub fn get_page(_: impl AsRef<str>) -> Result<String, Error> {
    Err(format_err!("unimplemented"))
}
