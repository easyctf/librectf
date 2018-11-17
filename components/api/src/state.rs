use std::sync::Arc;

use actix_web::{FromRequest, HttpRequest};
use failure::Error;

use super::{config::MailCredentials, Config, DbConn};
use core::{establish_connection, Pool};

struct InnerState {
    pub(super) mail_credentials: Option<MailCredentials>,
    pub(super) secret_key: Vec<u8>,
    pub(super) pool: Pool,
}

#[derive(Clone)]
pub struct State {
    inner: Arc<InnerState>,
}

impl State {
    pub fn from(config: Config) -> State {
        let pool = establish_connection(&config.database_url);
        let inner = InnerState {
            mail_credentials: config.mail_credentials,
            secret_key: config.secret_key.into_bytes(),
            pool,
        };
        let inner = Arc::new(inner);
        State { inner }
    }

    pub fn get_secret_key(&self) -> &Vec<u8> {
        &self.inner.secret_key
    }

    pub fn get_connection(&self) -> Result<DbConn, Error> {
        match self.inner.pool.get() {
            Ok(conn) => Ok(DbConn::new(conn)),
            Err(err) => Err(format_err!("Database connection error: {}", err)),
        }
    }

    pub fn get_file(&self, uri: &str) {}
}

impl FromRequest<State> for State {
    type Config = ();
    type Result = Result<Self, Error>;

    #[inline]
    fn from_request(req: &HttpRequest<State>, _: &Self::Config) -> Self::Result {
        Ok(req.state().clone())
    }
}
