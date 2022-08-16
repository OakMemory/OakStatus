use std::fmt::Debug;

use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome},
    Request,
};

use crate::{service::secret_bucket::TokenBucket, utils::constant};

#[derive(Default)]
pub struct SecretVertifyGuard;

#[derive(Debug)]
pub enum SecretVertifyError {
    NoSuchToken,
    NoToken,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SecretVertifyGuard {
    type Error = SecretVertifyError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if !req.headers().contains(constant::HEADER_OAK_PUSH_TOKEN) {
            return Outcome::Failure((Status::BadRequest, SecretVertifyError::NoToken));
        }

        let token = req
            .headers()
            .get(constant::HEADER_OAK_PUSH_TOKEN)
            .last()
            .unwrap();

        match TokenBucket::check(token).await {
            true => Outcome::Success(SecretVertifyGuard),
            false => Outcome::Failure((Status::NotAcceptable, SecretVertifyError::NoSuchToken)),
        }
    }
}
