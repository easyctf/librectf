use Backend;

#[macro_export]
macro_rules! schema {
    ( $(#[$attr:meta])* schema $schema:ident; $($struct:item)* ) => {
        $($attr)*
        pub struct $schema {

        }
        impl<'s> ::orm::Schema<'s, ::orm::MysqlBackend> for $schema {
        }
        $($struct)*
    };
}

/// Backend-agnostic schema.
pub trait Schema<'s, B: Backend + 's> {
}
