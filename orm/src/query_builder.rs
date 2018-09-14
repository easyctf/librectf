use {Backend, MysqlBackend};

pub trait QueryBuilder<B: Backend> {}

pub struct MysqlQueryBuilder {}

impl QueryBuilder<MysqlBackend> for MysqlQueryBuilder {}
