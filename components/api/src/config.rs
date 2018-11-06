#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MailCredentials {
    SMTP { host: String },
}

/// Web server specific config
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    /// Whether to run the application in debug mode
    #[serde(default)]
    pub debug: bool,

    /// The secret key used for signing cookies
    pub secret_key: String,

    /// The host to bind to
    pub bind_host: String,

    /// The port to bind to
    pub bind_port: u16,

    /// The URL for the database as a MySQL connection string.
    pub database_url: String,

    /// Credentials to use to send mail
    pub mail_credentials: Option<MailCredentials>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigWrapper {
    pub api: Config,
}
