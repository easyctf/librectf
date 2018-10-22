use std::path::PathBuf;

#[derive(Debug, StructOpt)]
pub struct FsConfig {
    /// Password for sending files to the filestore.
    #[structopt(long = "push-password", env = "FILESTORE_PUSH_PASSWORD")]
    push_password: String,

    /// Password for retrieving files from the protected filestore.
    #[structopt(long = "pull-password", env = "FILESTORE_PULL_PASSWORD")]
    pull_password: String,

    /// Location for storing files (must exist).
    #[structopt(
        long = "storage-dir",
        env = "FILESTORE_STORAGE_DIR",
        parse(from_os_str)
    )]
    storage_dir: PathBuf,


}
