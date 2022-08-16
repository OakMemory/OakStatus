use crate::{config::OakConfig, utils::instance::OakSingleton};

pub struct TokenBucket;

impl TokenBucket {
    pub async fn check(token: impl Into<String>) -> bool {
        let x = Self::get().await;

        match x.binary_search(&token.into()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    #[allow(dead_code)]
    pub async fn get() -> Vec<String> {
        OakConfig::get_instance()
            .read()
            .await
            .extract_inner::<Vec<String>>("secret_bucket")
            .unwrap()
    }
}
