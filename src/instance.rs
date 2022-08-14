use rocket::tokio::sync::Mutex;

pub trait OakSingleton {
    fn get_instance() -> &'static Mutex<Self>;
}
