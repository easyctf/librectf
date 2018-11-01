use api;
use filestore;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub bind_host: String,
    pub bind_port: u16,

    pub api: Option<api::Config>,
    pub filestore: Option<filestore::Config>,
}
