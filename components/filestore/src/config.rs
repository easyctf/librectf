use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Password for sending files to the filestore.
    pub push_password: String,

    /// Password for retrieving files from the protected filestore.
    pub pull_password: String,

    /// The prefix for output URLs
    pub url_prefix: String,

    /// The host to bind to
    pub bind_host: String,

    /// The port to bind to
    pub bind_port: u16,

    /// Location for storing files (must exist).
    pub storage_dir: PathBuf,
}
