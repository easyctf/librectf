#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// The URL for the database as a MySQL connection string.
    pub database_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigWrapper {
    pub admin: Config,
}
