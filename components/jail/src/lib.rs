use std::path::PathBuf;

pub struct JailOptions {

}

pub struct Jail {
    pub path: PathBuf,
    pub options: JailOptions,
}