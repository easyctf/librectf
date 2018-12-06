use diesel::{Connection, MysqlConnection};

pub fn establish_connection(database_url: impl AsRef<str>) -> Option<MysqlConnection> {
    let database_url = database_url.as_ref();
    MysqlConnection::establish(database_url).ok()
}
