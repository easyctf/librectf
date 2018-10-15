use actix_web::{
    self,
    http::header::HeaderValue,
    middleware::{Middleware, Response},
    HttpRequest, HttpResponse,
};

use super::State;

pub struct APIMiddleware;

impl Middleware<State> for APIMiddleware {
    fn response(
        &self,
        _: &HttpRequest<State>,
        mut res: HttpResponse,
    ) -> actix_web::Result<Response> {
        res.headers_mut()
            .insert("Cache-Control", HeaderValue::from_static("no-cache"));
        Ok(Response::Done(res))
    }
}
