use std::fmt::Debug;

use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};

#[derive(Default)]
pub struct RequestCountGuard;

#[derive(Debug)]
pub enum RequestCountError {
    OutOfRequest,
}
#[allow(unused_variables)]
#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestCountGuard {
    type Error = RequestCountError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Failure((Status::TooManyRequests, RequestCountError::OutOfRequest))
    }
}
