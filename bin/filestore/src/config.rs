use std::path::PathBuf;

#[derive(Clone, Debug, StructOpt)]
pub struct FsConfig {
    /// Password for sending files to the filestore.
    #[structopt(long = "push-password", env = "FILESTORE_PUSH_PASSWORD")]
    pub push_password: String,

    /// Password for retrieving files from the protected filestore.
    #[structopt(long = "pull-password", env = "FILESTORE_PULL_PASSWORD")]
    pub pull_password: String,

    /// The host to bind to
    #[structopt(
        long = "bind_host",
        env = "FILESTORE_BIND_HOST",
        default_value = "127.0.0.1"
    )]
    pub bind_host: String,

    /// The port to bind to
    #[structopt(
        long = "bind_port",
        env = "FILESTORE_BIND_PORT",
        default_value = "8001"
    )]
    pub bind_port: u16,

    /// Location for storing files (must exist).
    #[structopt(
        long = "storage-dir",
        env = "FILESTORE_STORAGE_DIR",
        parse(from_os_str)
    )]
    pub storage_dir: PathBuf,
}
