use BaseQuery;

pub trait Model: Sized {
    /// Begin building a query.
    fn query() -> BaseQuery<Self>;
}
