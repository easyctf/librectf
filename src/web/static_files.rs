//! Reimplementation of rocket_contrib's static_files module, except using
//! the custom Embed derive.

// TODO: expand this to do some kind of browser-level caching with headers
// TODO: expand this to allow extra paths in the search (besides precompiled)

use std::borrow::Cow;
use std::io::Cursor;

use mime_guess::guess_mime_type;
use rocket::{
    handler::Outcome,
    http::{ContentType, Header, Method},
    response::{Body, Content},
    Response, Route,
};

#[derive(Default, Clone, Embed)]
#[folder = "static"]
pub struct StaticFiles;

impl Into<Vec<Route>> for StaticFiles {
    fn into(self) -> Vec<Route> {
        vec![Route::ranked(10, Method::Get, "/<path..>", |req, _| {
            let path = req.uri().path();
            let path = path.trim_left_matches("/static/");

            // sorry sergio, i'm gonna use mime_guess here
            let ct = {
                let mime = guess_mime_type(&path);
                let top = Cow::from(String::from(mime.type_().as_ref()));
                let bottom = Cow::from(String::from(mime.subtype().as_ref()));
                ContentType::new(top, bottom)
            };
            let response = match StaticFiles::get(&path) {
                Some(resource) => {
                    let len = resource.len() as u64;
                    let response = Response::build()
                        .raw_body(Body::Sized(Cursor::new(resource.clone()), len))
                        .header(Header::new("Cache-Control", "max-age=31536000"))
                        .finalize();
                    Some(Content(ct, response))
                }
                None => None,
            };
            Outcome::from(req, response)
        })]
    }
}
