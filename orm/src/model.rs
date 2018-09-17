#[derive(Default)]
pub struct Entity {}

pub trait IntoEntities<T> {
    fn into_entities(self) -> Vec<Entity>;
}

pub trait Model: Sized {}

#[derive(Default)]
pub struct Column<T> {
    _inner: ::std::marker::PhantomData<T>,
}

impl<T> IntoEntities<Column<T>> for Column<T> {
    fn into_entities(self) -> Vec<Entity> {
        Vec::new()
    }
}

macro_rules! impl_into_entities {
    ($(
        $Tuple:tt {
            $(($idx:tt) -> $T:ident, $ST:ident, $TT:ident,)+
        }
    )+) => {
        $(
            impl<$($ST,)+ $($T: IntoEntities<$ST>,)+> IntoEntities<($($ST,)+)> for ($($T,)+) {
                fn into_entities(self) -> Vec<Entity> {
                    let mut v = Vec::new();
                    $(v.extend(self.$idx.into_entities());)+
                    v
                }
            }
        )+
    }
}

__diesel_for_each_tuple!(impl_into_entities);
