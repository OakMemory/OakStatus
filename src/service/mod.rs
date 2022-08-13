use std::sync::Mutex;

pub mod request_counter_service;

pub trait OakService {
    fn get_instance() -> &'static Mutex<Self>;
}
