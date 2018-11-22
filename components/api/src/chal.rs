use comrak::{
    format_html,
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use core::models::{Challenge, NewSolve};
use diesel::prelude::*;
use failure::Error;

use super::DbConn;

// TODO: return a "challenge entry" struct instead of the original challenge
pub fn list_all(db: DbConn) -> Result<Vec<Challenge>, Error> {
    use core::schema::chals::dsl::*;
    chals
        .load::<Challenge>(&*db)
        .map(|list| {
            list.into_iter()
                .map(|mut chal| {
                    let arena = Arena::new();
                    let desc = parse_document(&arena, &chal.description, &ComrakOptions::default());

                    fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
                    where
                        F: Fn(&'a AstNode<'a>),
                    {
                        f(node);
                        for c in node.children() {
                            iter_nodes(c, f);
                        }
                    }

                    iter_nodes(desc, &|node| match &mut node.data.borrow_mut().value {
                        &mut NodeValue::Link(ref mut link)
                        | &mut NodeValue::Image(ref mut link) => {
                            // TODO: look up file URL here
                            link.url = String::from("https://www.example.com").into_bytes();
                        }
                        _ => (),
                    });

                    let mut html = Vec::new();
                    format_html(desc, &ComrakOptions::default(), &mut html).unwrap();

                    chal.description = String::from_utf8(html).unwrap();
                    chal
                }).collect::<Vec<_>>()
        }).map_err(|err| err.into())
}

#[derive(Serialize, Deserialize)]
pub struct SubmitForm {
    pub id: i32,
    pub flag: String,
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

        let judgment = if if chal.regex {
            let rgx = Regex::new(&chal.correct_flag).unwrap();
            rgx.is_match(&form.flag)
        } else {
            form.flag == chal.correct_flag
        } {
            // TODO: award points
            SubmissionResult::Correct
        } else {
            // TODO: count up incorrect solves?
            SubmissionResult::Incorrect
        };

        // insert solve
        let new_solve = NewSolve {
            flag: form.flag,
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
