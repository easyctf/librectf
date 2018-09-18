//! Structured query representation.

mod operator;
mod select;

pub use self::operator::Operator;
pub use self::select::Select;

use Backend;

pub trait QueryClause<B: Backend> {
    fn to_query_string(&self) -> String;
}
