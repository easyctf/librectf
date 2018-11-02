use actix_web::{HttpRequest, HttpResponse, Responder};
use failure::Error;
use futures::Future;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AsyncTask<T> {
    inner: T,
}
