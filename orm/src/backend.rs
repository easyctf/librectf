use {MysqlQueryBuilder, QueryBuilder};

pub trait Backend
where
    Self: Sized,
{
    type QueryBuilder: QueryBuilder<Self>;
}

pub struct MysqlBackend {}

impl Backend for MysqlBackend {
    type QueryBuilder = MysqlQueryBuilder;
}
