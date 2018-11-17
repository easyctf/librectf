use core::models::{Challenge, NewSolve};
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
    use diesel::result::Error::RollbackTransaction;
    use regex::Regex;

    db.transaction(|| {
        let chal = match chals.filter(id.eq(form.id)).first::<Challenge>(&*db) {
            Ok(chal) => chal,
            Err(err) => {
                error!("Diesel error on flag submission: {}", err);
                return Err(RollbackTransaction);
            }
        };

        let judgment = if {
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
        };

        // insert solve
        let new_solve = NewSolve {
            flag: form.cand.clone(),
            chal_id: chal.id,
            // TODO: get submitter information here
            team_id: 1,
            user_id: 12,
        };
        if let Err(err) = {
            use core::schema::solves;
            diesel::insert_into(solves::table)
                .values(&new_solve)
                .execute(&*db)
        } {
            error!("Diesel error on solve insertion: {}", err);
            return Err(RollbackTransaction);
        }

        Ok(judgment)
    }).map_err(|err| err.into())
}
