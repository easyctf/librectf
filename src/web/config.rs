#[derive(Clone, Debug, StructOpt)]
pub struct WebConfig {
    /// Whether to run the application in debug mode
    #[structopt(long = "debug", env = "DEBUG")]
    pub debug: bool,

    /// The secret key used for signing cookies
    #[structopt(long = "secret-key", env = "SECRET_KEY")]
    pub secret_key: String,

    /// SMTP server host
    #[structopt(long = "smtp-host", env = "SMTP_HOST")]
    pub smtp_host: Option<String>,

    /// The host to bind to
    #[structopt(
        long = "bind_host",
        env = "BIND_HOST",
        default_value = "127.0.0.1"
    )]
    pub bind_host: String,

    /// The port to bind to
    #[structopt(
        long = "bind_port",
        env = "BIND_PORT",
        default_value = "8000"
    )]
    pub bind_port: u16,

    /// The URL for the database as a MySQL connection string.
    #[structopt(long = "database-url", env = "DATABASE_URL")]
    pub database_url: String,
}
