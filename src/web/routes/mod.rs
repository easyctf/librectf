macro_rules! generate_form_field {
    ($value:ident => $name:ident ($($args:tt)*) {$($body:tt)*}) => {
        struct $name ($($args)*);

        impl<'v> ::rocket::request::FromFormValue<'v> for $name {
            type Error = String;
            fn from_form_value($value: &'v ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                let $value: &str = $value.as_ref();
                $($body)*
            }
        }
    };
}

pub mod base;
pub mod user;
