use super::QueryClause;
use Backend;

pub struct Select {
    from_clause: Option<SelectFromClause>,
    where_clause: SelectWhereClause,
    inner_columns: Vec<SelectColumn>,
}

pub struct SelectColumn {}

pub struct SelectFromClause {}

pub enum SelectWhereClause {}
