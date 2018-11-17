use diesel::{Connection, MysqlConnection};

use config::Config;

pub fn establish_connection(config: &Config) -> Option<MysqlConnection> {
    match &config.database_url {
        Some(database_url) => Some(
            MysqlConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", &database_url)),
        ),
        None => None,
    }
}
