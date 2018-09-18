use ConnectionPool;
use Entity;
use IntoEntities;

pub struct Query<'a, T> {
    connection: &'a ConnectionPool,
    entities: Vec<Entity>,
    _inner: ::std::marker::PhantomData<T>,
}

pub struct QueryResult {}

pub enum Value {}

impl<'a, T> Query<'a, T> {
    pub fn new(connection: &'a ConnectionPool, ent: impl IntoEntities<T>) -> Self {
        Query {
            connection,
            entities: ent.into_entities(),
            _inner: ::std::marker::PhantomData::default(),
        }
    }

    pub fn all(&self) {

    }
}
