//! Functions associated with dealing with team manipulation.

use wtforms::Form;

use crate::db::DbConn;
use crate::models::{NewTeam, Team};
use crate::Error;

/// The struct behind the form used in the team creation page.
#[derive(Form, Serialize, Deserialize)]
#[allow(missing_docs)]
pub struct CreateForm {
    pub name: String,
    pub affiliation: Option<String>,
}

/// Attempts to create a new team, returning the ID of the newly created team.
pub fn create_team(db: &DbConn, creator: i32, form: &CreateForm) -> Result<i32, Error> {
    let new_team = NewTeam {
        name: form.name.clone(),
        captain_id: creator,
    };
    db.create_team(&new_team)
}
