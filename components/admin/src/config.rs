#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// The URL for the database as a MySQL connection string
    pub database_url: Option<String>,

    /// The URL of the filestore
    pub filestore_url: Option<String>,

    /// Upload password for filestore
    pub filestore_push_password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigWrapper {
    pub admin: Config,
}
