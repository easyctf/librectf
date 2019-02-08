//! Database models.

use super::schema::*;

/// Used for inserting challenges into the database.
#[derive(Debug, Insertable)]
#[table_name = "chals"]
#[doc(hidden)]
pub struct NewChallenge {
    pub title: String,
    pub enabled: bool,
    pub description: String,
    pub correct_flag: String,
    pub regex: bool,
    pub value: i32,
}

/// Represents a single challenge.
#[derive(Debug, Queryable, Serialize)]
pub struct Challenge {
    /// Numerical id of the challenge.
    pub id: i32,

    /// The title of the challenge as it appears on the listing.
    pub title: String,

    /// Whether or not the challenge is open for solving. You can toggle this
    /// field to temporarily disable challenges.
    pub enabled: bool,

    /// The description (in Markdown syntax). This will get re-rendered into
    /// proper HTML (and possibly be used in a template) before it appears in
    /// the listing.
    pub description: String,

    /// The correct flag. If the `regex` field is set to `true`, then this
    /// string will be compiled into a regular expression and the flag will be
    /// matched against the regular expression. Otherwise, this field will be
    /// compared against the flag in a traditional string comparison.
    pub correct_flag: String,

    /// Whether or not to enable regex compilation for the `correct_flag`.
    pub regex: bool,

    /// How much this challenge is worth (points).
    pub value: i32,
}

/// An invitation from a team to a user to join that team.
#[derive(Debug, Queryable, Insertable)]
#[allow(missing_docs)]
pub struct Invitation {
    pub team_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Insertable)]
#[table_name = "files"]
#[doc(hidden)]
pub struct NewFile {
    pub name: String,
    pub url: String,
    pub chal_id: i32,
    pub team_id: Option<i32>,
}

/// A file that's referenced by a challenge.
#[derive(Debug, Queryable)]
pub struct File {
    /// Numerical id of the file.
    pub id: i32,

    /// A string that, in conjunction with the problem id, can be used to
    /// uniquely identify this file.
    pub name: String,

    /// The URL at which this file appears.
    pub url: String,

    /// The id of the challenge with which this file is associated.
    pub chal_id: i32,

    /// Optionally, the team for which this file was generated. This is useful
    /// for autogen challenges where each team receives a different version of
    /// the file.
    pub team_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
#[doc(hidden)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

/// A user.
#[derive(Clone, Debug, Queryable, Serialize)]
pub struct User {
    /// Numerical id of the user.
    pub id: i32,

    /// Display name.
    pub name: String,

    /// Email.
    pub email: String,

    /// Whether or not the user's email has been verified.
    pub email_verified: bool,

    /// Bcrypt-hashed password.
    #[serde(skip)]
    pub password: String,

    /// Whether or not this user has administrator privileges.
    pub admin: bool,

    /// The team that this user belongs to(if the user belongs to a team).
    pub team_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "teams"]
#[doc(hidden)]
pub struct NewTeam {
    pub name: String,
    pub captain_id: i32,
}

/// A team.
#[derive(Clone, Debug, Queryable)]
pub struct Team {
    /// Numerical id of the team.
    pub id: i32,

    /// The user id of the "owner" of the team. This user is allowed to make
    /// administrative changes to the team, like changing the team name or
    /// inviting other users.
    pub captain_id: i32,

    /// The team name.
    pub name: String,

    /// An extra field associated with the team.
    pub affiliation: Option<String>,

    /// Whether or not the team is banned. If the team is banned, they will not
    /// appear on the scoreboard. (TODO: probably replace this with a status
    /// enum instead)
    pub banned: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "solves"]
#[doc(hidden)]
pub struct NewSolve {
    pub flag: String,
    pub chal_id: i32,
    pub team_id: i32,
    pub user_id: i32,
}
