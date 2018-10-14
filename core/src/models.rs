use super::schema::*;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub password: String,
    pub admin: bool,
    pub team_id: Option<i32>,
}

#[derive(Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub affiliation: Option<String>,
    pub banned: bool,
}
