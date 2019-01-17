use std::collections::HashMap;

use comrak::{
    format_html,
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use core::models::{Challenge, File, NewSolve};
use diesel::prelude::*;

use super::DbConn;

// TODO: return a "challenge entry" struct instead of the original challenge
pub fn list_all(db: DbConn) -> Result<Vec<Challenge>, Error> {
    use core::schema::chals::dsl::*;

    let lookup = {
        use core::schema::files::dsl::files;
        files
            .load::<File>(&*db)
            .map_err(|err| <_ as Into<Error>>::into(err))
            .map(|list| {
                let mut chal_map = HashMap::new();
                for item in list {
                    if let None = chal_map.get(&item.chal_id) {
                        chal_map.insert(item.chal_id, HashMap::new());
                    }

                    chal_map
                        .get_mut(&item.chal_id)
                        .map(|file_map| file_map.insert(item.name.clone().into_bytes(), item.url));
                }
                chal_map
            })?
    };

    chals
        .load::<Challenge>(&*db)
        .map(|list| {
            let mut list = list
                .into_iter()
                .map(|mut chal| {
                    let arena = Arena::new();
                    let desc = parse_document(&arena, &chal.description, &ComrakOptions::default());

                    if let Some(file_map) = lookup.get(&chal.id) {
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
                                if let Some(url) = file_map.get(&link.url) {
                                    link.url = url.clone().into_bytes();
                                }
                            }
                            _ => (),
                        });
                    }

                    let mut html = Vec::new();
                    format_html(desc, &ComrakOptions::default(), &mut html).unwrap();

                    chal.description = String::from_utf8(html).unwrap();
                    chal
                }).collect::<Vec<_>>();
            list.sort_unstable_by(|a, b| a.value.cmp(&b.value));
            list
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

pub struct Submission {
    pub user_id: i32,
    pub team_id: i32,
    pub form: SubmitForm,
}

pub fn submit_flag(db: DbConn, submission: Submission) -> Result<SubmissionResult, Error> {
    use core::schema::chals::dsl::*;
    use diesel::result::Error::RollbackTransaction;
    use regex::Regex;

    db.transaction(|| {
        let chal = match chals
            .filter(id.eq(submission.form.id))
            .first::<Challenge>(&*db)
        {
            Ok(chal) => chal,
            Err(err) => {
                error!("Diesel error on flag submission: {}", err);
                return Err(RollbackTransaction);
            }
        };

        let judgment = if if chal.regex {
            let rgx = Regex::new(&chal.correct_flag).unwrap();
            rgx.is_match(&submission.form.flag)
        } else {
            submission.form.flag == chal.correct_flag
        } {
            // TODO: award points
            SubmissionResult::Correct
        } else {
            // TODO: count up incorrect solves?
            SubmissionResult::Incorrect
        };

        // insert solve
        let new_solve = NewSolve {
            flag: submission.form.flag,
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
