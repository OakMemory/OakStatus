#[cfg(test)]
mod test {
    use rocket::tokio;

    use crate::{config::OakConfigContent, service::secret_bucket::TokenBucket};

    #[tokio::main]
    #[test]
    async fn get() {
        let x = TokenBucket::get().await.clone();
        assert_eq!(x.len(), OakConfigContent::default().secret_bucket.len())
    }

    #[tokio::main]
    #[test]
    async fn check() {
        assert_eq!(
            TokenBucket::check(OakConfigContent::default().secret_bucket.first().unwrap()).await,
            true
        )
    }
}
