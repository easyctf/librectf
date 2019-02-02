use core::Error;
use tera::Context;
use warp::Filter;
use wtforms::Form;

use crate::render::render_template;

#[derive(Form, Serialize, Deserialize)]
pub struct RegisterForm {
    pub name: String,
}

pub fn get_register() -> Resp!() {
    warp::any().and_then(|| render_template("users/register.html", Context::new()))
}

pub fn post_register() -> Resp!() {
    warp::any()
        .and(warp::body::form())
        .and_then(|form: RegisterForm| {
            form.validate()
                .map_err(Error::from)
                .map_err(warp::reject::custom)
        })
        .map(|_| "hi")
}
