use super::schema::*;

#[derive(Debug, Insertable)]
#[table_name = "chals"]
pub struct NewChallenge {
    pub title: String,
    pub enabled: bool,
    pub description: String,
    pub correct_flag: String,
    pub regex: bool,
    pub value: i32,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Challenge {
    pub id: i32,
    pub title: String,
    pub enabled: bool,
    pub description: String,
    pub correct_flag: String,
    pub regex: bool,
    pub value: i32,
}

#[derive(Debug, Queryable, Insertable)]
pub struct Invitation {
    pub team_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "files"]
pub struct NewFile {
    pub name: String,
    pub url: String,
    pub chal_id: i32,
    pub team_id: Option<i32>,
}

#[derive(Debug, Queryable)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub chal_id: i32,
    pub team_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    #[serde(skip)]
    pub password: String,
    pub admin: bool,
    pub team_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "teams"]
pub struct NewTeam {
    pub name: String,
    pub captain_id: i32,
}

#[derive(Debug, Queryable)]
pub struct Team {
    pub id: i32,
    pub captain_id: i32,
    pub name: String,
    pub affiliation: Option<String>,
    pub banned: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "solves"]
pub struct NewSolve {
    pub flag: String,
    pub chal_id: i32,
    pub team_id: i32,
    pub user_id: i32,
}