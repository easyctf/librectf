use failure::Error;

use DbConn;

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteUserForm {
    user_id: i32,
}

pub fn invite_user(db: DbConn, form: InviteUserForm) -> Result<(), Error> {
    Ok(())
}
