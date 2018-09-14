use AsQuery;
use Backend;

pub struct BaseQuery<T> {
    _x: ::std::marker::PhantomData<T>,
}

pub trait QueryBuilder<B: Backend>
where
    Self: Default,
{
    fn build(&self) -> String;
}

impl<T> BaseQuery<T> {
    pub fn new() -> Self {
        BaseQuery { _x: ::std::marker::PhantomData::default() }
    }
}

impl<T> AsQuery for BaseQuery<T> {
    type Query = String;
    fn as_query(&self) -> Self::Query { String::new() }
}