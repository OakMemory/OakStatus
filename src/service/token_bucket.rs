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
    fn get_instance() -> &'static rocket::tokio::sync::Mutex<Self> {
        todo!()
    }
}
