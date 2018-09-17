#[cfg(feature = "mysql")]
pub mod mysql;

use r2d2::ManageConnection;

pub trait Backend
where
    Self: Sized,
{
    type ConnectionManager: ManageConnection;
}
