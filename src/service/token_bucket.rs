use once_cell::sync::OnceCell;
use rocket::tokio::sync::RwLock;

use crate::utils::instance::OakSingleton;

pub type TokenBucket = Vec<String>;

pub trait TokenBucketTrait {
    fn check(&self, token: impl Into<String>) -> bool;
    fn put(&mut self, token: impl Into<String>) -> bool;
}

impl TokenBucketTrait for TokenBucket {
    fn check(&self, token: impl Into<String>) -> bool {
        match self.binary_search(&token.into()) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn put(&mut self, token: impl Into<String>) -> bool {
        let token = Box::new(token.into());

        match self.check(token.to_string()) {
            true => {
                self.push(token.to_string());
                true
            }
            false => false,
        }
    }
}

impl OakSingleton for TokenBucket {
    fn get_instance() -> &'static rocket::tokio::sync::RwLock<Self> {
        static INSTANCE: OnceCell<RwLock<TokenBucket>> = OnceCell::new();
        INSTANCE.get_or_init(|| RwLock::new(TokenBucket::default()))
    }
}
