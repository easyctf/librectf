/// OpenCTF main configuration
// TODO: remove Default
#[derive(Config, Default)]
pub struct Config {
    /// The URL for the database as a MySQL connection string.
    pub database_url: String,
}
