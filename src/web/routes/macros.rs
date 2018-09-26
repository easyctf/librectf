macro_rules! generate_form_field {
    ($value:ident => $vis:vis $name:ident ($($args:tt)*) {$($body:tt)*}) => {
        $vis struct $name ($($args)*);

        impl<'v> ::rocket::request::FromFormValue<'v> for $name {
            type Error = String;
            fn from_form_value($value: &'v ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                let $value: String = $value.url_decode().map_err(|err| format!("URL encoding error: {}", err))?;
                $($body)*
            }
        }
    };
}

macro_rules! generate_form {
    ($vis:vis $name:ident { $( $member:ident : $ty:ty ,)* }) => {
        #[derive(FromForm)]
        #[allow(dead_code)]
        $vis struct $name {
            $(
                $member: Result<$ty, String>,
            )*
        }
    };

    ($vis:vis $name:ident => $into:ident { $( $member2:ident = $member:ident : $ty:ty ,)* }) => {
        #[derive(FromForm)]
        #[allow(dead_code)]
        $vis struct $name {
            $(
                $member: Result<$ty, String>,
            )*
        }

        impl<'a> TryFrom<&'a $name> for $into {
            type Error = Vec<String>;
            fn try_from(form: &'a $name) -> Result<Self, Self::Error> {
                let mut errors = Vec::new();
                $(
                    let $member = match &form.$member {
                        Ok(ref $member) => $member.0.clone(),
                        Err(ref err) => {
                            errors.push(err.clone());
                            String::new()
                        }
                    };
                )*
                if !errors.is_empty() {
                    return Err(errors);
                }
                Ok($into {
                    $(
                        $member2: $member,
                    )*
                })
            }
        }
    }
}
