use rocket::{http::Status, response::Responder, Request, Response};

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<'r, A: Responder<'r>, B: Responder<'r>> Responder<'r> for Either<A, B> {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        match self {
            Either::Left(resp) => resp.respond_to(req),
            Either::Right(resp) => resp.respond_to(req),
        }
    }
}
