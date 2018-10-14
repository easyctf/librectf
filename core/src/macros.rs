#[macro_export]
macro_rules! error_wrapper {
    ($name:ident: $cause:path = $v:expr) => {
        #[derive(Debug, Fail)]
        #[fail(display = $v)]
        pub struct $name(#[cause] pub $cause);
    };
}

#[macro_export]
macro_rules! error_derive_from {
    ($error_type:ident = {$($error:path[$v:expr] => $into:ident,)*}) => {
        #[derive(Debug)]
        pub struct CustomError(String);

        impl CustomError {
            pub fn new(s: impl AsRef<str>) -> Self {
                CustomError(s.as_ref().to_owned())
            }
        }

        impl ::std::fmt::Display for CustomError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::failure::Fail for CustomError {}

        impl From<CustomError> for $error_type {
            fn from(err: CustomError) -> Self {
                $error_type::Custom(err)
            }
        }

        #[derive(Debug, Fail)]
        #[allow(dead_code)]
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
