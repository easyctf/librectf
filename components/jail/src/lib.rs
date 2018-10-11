use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Default)]
pub struct JailBuilder {
    path: PathBuf,
}

impl JailBuilder {
    pub fn new(path: impl AsRef<Path>) -> JailBuilder {
        JailBuilder {
            path: path.as_ref().to_path_buf(),
            ..Default::default()
        }
    }

    pub fn build(&self) -> Command {
        Command::new("nsjail")
    }
}
