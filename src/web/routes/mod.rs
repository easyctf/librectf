use rocket::{
    http::{ContentType, Status},
    response::{Content, Responder, Response},
    Request,
};
use serde::Serialize;
use serde_json::Value as JsonValue;

use Error;

#[macro_use]
mod macros;

pub mod base;
pub mod team;
pub mod user;

pub struct JSON(JsonValue);

impl JSON {
    pub fn new(v: impl Serialize) -> Result<Self, Error> {
        serde_json::to_value(v)
            .map(|v| JSON(v))
            .map_err(|err| err.into())
    }
}

impl<'r> Responder<'r> for JSON {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        Content(ContentType::JsonApi, serde_json::to_string(&self.0)).respond_to(req)
    }
}
