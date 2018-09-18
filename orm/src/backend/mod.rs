#[cfg(feature = "mysql")]
pub mod mysql;

use Compiler;
use r2d2::ManageConnection;

pub trait Backend
where
    Self: Sized,
{
    type Compiler: Compiler;
    type ConnectionManager: ManageConnection;
}
