#[cfg(feature = "mysql")]
pub mod mysql;

use r2d2::ManageConnection;

use QueryBuilder;

pub trait Backend
where
    Self: Sized,
{
    type QueryBuilder: QueryBuilder<Self>;
    type ConnectionManager: ManageConnection;
}
