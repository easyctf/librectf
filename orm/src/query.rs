use ConnectionPool;
use Entity;
use IntoEntities;

pub struct Query<'a, T> {
    connection: &'a ConnectionPool,
    entities: Vec<Entity>,
    _inner: ::std::marker::PhantomData<T>,
}

pub struct QueryResult {

}

pub enum Value {}

impl<'a, T> Query<'a, T> {
    pub fn new(connection: &'a ConnectionPool, ent: impl IntoEntities<T>) -> Self {
        Query {
            connection,
            entities: ent.into_entities(),
            _inner: ::std::marker::PhantomData::default(),
        }
    }
}

impl<'a, T> IntoIterator for Query<'a, T> {
    type Item = Value;
    type IntoIter = QueryResult;
    fn into_iter(self) -> Self::IntoIter {
        QueryResult {}
    }    
}

impl Iterator for QueryResult {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
