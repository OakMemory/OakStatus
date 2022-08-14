use std::fmt::Debug;

use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};

use crate::{
    service::token_bucket::{TokenBucket, TokenBucketTrait},
    utils::{constant, instance::OakSingleton},
};

#[derive(Default)]
pub struct TokenVertify;

#[derive(Debug)]
pub enum TokenVertifyError {
    NoSuchToken,
    NoToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for TokenVertify {
    type Error = TokenVertifyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if req.headers().contains(constant::HEADER_OAK_PUSH_TOKEN) {
            return Outcome::Failure((Status::BadRequest, TokenVertifyError::NoToken));
        }

        let token = req
            .headers()
            .get(constant::HEADER_OAK_PUSH_TOKEN)
            .last()
            .unwrap();

        match TokenBucket::get_instance().lock().await.check(token) {
            true => Outcome::Success(TokenVertify),
            false => Outcome::Failure((Status::NotAcceptable, TokenVertifyError::NoSuchToken)),
        }
    }
}
