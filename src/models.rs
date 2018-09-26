use super::schema::*;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub password: String,
}
