macro_rules! error_wrapper {
    ($name:ident: $cause:path = $v:expr) => {
        #[derive(Debug, Fail)]
        #[fail(display = $v)]
        pub struct $name(#[cause] pub $cause);
    };
}

macro_rules! error_derive_from {
    ($error_type:ident = {$($error:path[$v:expr] => $into:ident,)*}) => {
        #[derive(Debug, Fail)]
        pub enum $error_type {
            #[fail(display = "{}", _0)]
            Custom(#[cause] ::errors::CustomError),
            $(
                #[fail(display = $v)]
                $into(#[cause] $error),
            )*
        }

        $(
            impl From<$error> for $error_type {
                fn from(err: $error) -> Self {
                    $error_type::$into(err)
                }
            }
        )*
    };
}
