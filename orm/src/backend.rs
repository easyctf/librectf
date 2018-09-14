use QueryBuilder;

pub trait Backend
where
    Self: Sized,
{
    type QueryBuilder: QueryBuilder<Self>;
}
