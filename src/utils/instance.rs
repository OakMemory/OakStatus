use rocket::tokio::sync::RwLock;

pub trait OakSingleton {
    fn get_instance() -> &'static RwLock<Self>;
}
