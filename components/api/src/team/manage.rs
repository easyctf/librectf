use core::models::Invitation;
use diesel::{prelude::*, result::Error::RollbackTransaction};

use DbConn;

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteUserForm {
    pub user_id: i32,
}

pub fn invite_user(db: DbConn, team_id: i32, form: InviteUserForm) -> Result<(), Error> {
    use core::schema::invitations::dsl::invitations;
    db.transaction(|| {
        let new_invitation = Invitation {
            team_id: team_id,
            user_id: form.user_id,
        };

        match diesel::insert_into(invitations)
            .values(&new_invitation)
            .execute(&*db)
        {
            Ok(_) => (),
            Err(err) => {
                error!("Error creating invitation: {}", err);
                return Err(RollbackTransaction);
            }
        };

        Ok(())
    }).map_err(|err| err.into())
}
