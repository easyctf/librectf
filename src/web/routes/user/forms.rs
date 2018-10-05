use std::convert::TryFrom;

use regex::Regex;

use models::NewUser;

lazy_static! {
    static ref USERNAME_PATTERN: Regex = Regex::new(r"[A-Za-z_][A-Za-z0-9_]{2,}").unwrap();
    static ref EMAIL_PATTERN: Regex = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();
}

generate_form_field!(value => UsernameField(pub String) {
    if !USERNAME_PATTERN.is_match(&value) {
        return Err("Invalid username (must be 3-20 chars and begin with a non-numeric character).".to_string());
    }
    Ok(UsernameField(value.to_owned()))
});

generate_form_field!(value => EmailField(pub String) {
    if !EMAIL_PATTERN.is_match(&value) {
        return Err("Invalid email.".to_string());
    }
    Ok(EmailField(value.to_owned()))
});

generate_form_field!(value => RegisterPasswordField(String) {
    let hashed = bcrypt::hash(&value, bcrypt::DEFAULT_COST).unwrap();
    Ok(RegisterPasswordField(hashed))
});

generate_form_field!(value => LoginPasswordField(String) {
    Ok(LoginPasswordField(value))
});

generate_form!(pub RegisterForm => NewUser {
    name = username: UsernameField,
    email = email: EmailField,
    password = password: RegisterPasswordField,
});

pub struct _LoginForm {
    pub email: String,
    pub password: String,
}

generate_form!(pub LoginForm => _LoginForm {
    email = email: EmailField,
    password = password: LoginPasswordField,
});
