use Backend;

#[macro_export]
macro_rules! schema {
    ( $(#[$attr:meta])* $v:vis schema $schema:ident; $($struct:item)* ) => {
        use ::orm_derive::schema_attr;
        #[allow(non_snake_case)]
        #[schema_attr($($attr,)*)]
        mod $schema {
            $($struct)*
        }
        $v use self::$schema::*;
    };
}

/// Backend-agnostic schema.
pub trait Schema<'s, B: Backend + 's> {}
