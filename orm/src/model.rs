use Backend;

pub trait Model<'s, B: Backend> {
    /// Begin building a query.
    fn query<T: Backend>(&self) -> <T as Backend>::QueryBuilder;
}
