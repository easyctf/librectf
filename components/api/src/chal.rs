use core::models::Challenge;
use diesel::prelude::*;
use failure::Error;

use super::DbConn;

pub fn list_all(db: DbConn) -> Result<Vec<Challenge>, Error> {
    use core::schema::chals::dsl::*;
    chals.load::<Challenge>(&*db).map_err(|err| err.into())
}

#[derive(Serialize, Deserialize)]
pub struct SubmitForm {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub enum SubmissionResult {
    Correct,
    Incorrect(String),
}

pub fn submit_flag(db: DbConn, form: SubmitForm) -> Result<SubmissionResult, Error> {
    use core::schema::chals::dsl::*;
    chals
        .filter(id.eq(form.id))
        .first::<Challenge>(&*db)
        .map_err(|err| err.into())
        .map(|chal| SubmissionResult::Correct)
}
