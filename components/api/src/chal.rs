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
    pub cand: String,
}

#[derive(Serialize, Deserialize)]
pub enum SubmissionResult {
    Correct,
    Incorrect,
}

pub fn submit_flag(db: DbConn, form: SubmitForm) -> Result<SubmissionResult, Error> {
    use core::schema::chals::dsl::*;
    use regex::Regex;
    chals
        .filter(id.eq(form.id))
        .first::<Challenge>(&*db)
        .map_err(|err| err.into())
        .map(|chal| {
            if {
                if chal.regex {
                    let rgx = Regex::new(&chal.correct_flag).unwrap();
                    rgx.is_match(&form.cand)
                } else {
                    form.cand == chal.correct_flag
                }
            } {
                // TODO: award points
                SubmissionResult::Correct
            } else {
                // TODO: count up incorrect solves?
                SubmissionResult::Incorrect
            }
        })
}
