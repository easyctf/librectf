/// OpenCTF main configuration
// TODO: remove Default
#[derive(Config, Default)]
pub struct Config {
    /// The URL for the database as a MySQL connection string.
    #[config(arg = "database-url", env = "DATABASE_URL", key = "database_url")]
    pub database_url: String,
}
