pub trait Query {
    type SqlType;
}

pub trait AsQuery {
    type Query;
    fn as_query(&self) -> Self::Query;
}
