/// OpenCTF main configuration

#[derive(Debug, StructOpt)]
pub struct Config {
    /// The URL for the database as a MySQL connection string.
    #[structopt(long = "database-url", env = "DATABASE_URL")]
    pub database_url: String,
}
