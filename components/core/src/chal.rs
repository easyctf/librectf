// TODO: filters?

use std::collections::HashMap;

use diesel::{prelude::*, result::Error::RollbackTransaction};
use comrak::{
    format_html,
    nodes::{AstNode, NodeValue},
    parse_document, Arena, ComrakOptions,
};
use failure::Error;
use regex::Regex;

use models::{Challenge, File, NewSolve};
use db::Connection as DbConn;

pub fn list_all(db: DbConn) -> Result<Vec<Challenge>, Error> {
    use schema::chals::dsl::*;

    let lookup = {
        use schema::files::dsl::files;
        files
            .load::<File>(&*db)
            .map_err(<_ as Into<Error>>::into)
            .map(|list| {
                let mut chal_map = HashMap::new();
                for item in list {
                    if chal_map.get(&item.chal_id).is_none() {
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
                            NodeValue::Link(ref mut link)
                            | NodeValue::Image(ref mut link) => {
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

pub struct Submission {
    pub user_id: i32,
    pub team_id: i32,
    pub form: SubmitForm,
}

pub fn submit(db: DbConn, submission: Submission) -> Result<Result<(), ()>, Error> {
    use schema::chals::dsl::*;
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
            Ok(())
        } else {
            // TODO: count up incorrect solves?
            Err(())
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
            use schema::solves;
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
