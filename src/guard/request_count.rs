use std::fmt::Debug;

use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};

use crate::{
    config::OakConfig, service::request_counter::RequestCounterService,
    utils::instance::OakSingleton,
};

#[derive(Default)]
pub struct RequestCountGuard;

#[derive(Debug)]
pub enum RequestCountError {
    OutOfRequest,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestCountGuard {
    type Error = RequestCountError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let request_limit: usize = OakConfig::get_instance()
            .read()
            .await
            .extract_inner("request_limit")
            .unwrap();

        let x = RequestCounterService::get_instance().read().await.get();

        if request_limit > RequestCounterService::get_instance().read().await.get() {
            Outcome::Success(Self)
        } else {
            Outcome::Failure((Status::TooManyRequests, RequestCountError::OutOfRequest))
        }
    }
}
