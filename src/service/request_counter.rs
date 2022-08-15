use std::{thread, time::Duration};

use once_cell::sync::OnceCell;
use rocket::tokio::sync::RwLock;

use crate::utils::instance::OakSingleton;

#[derive(Debug, Default)]
pub struct RequestCounterService {
    pub count: i128,
}

impl RequestCounterService {
    pub fn increase(&mut self) {
        self.count = self.count + 1;
    }

    pub fn clean(&mut self) {
        self.count = 0;
    }
}

impl OakSingleton for RequestCounterService {
    fn get_instance() -> &'static RwLock<RequestCounterService> {
        static INSTANCE: OnceCell<RwLock<RequestCounterService>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            thread::spawn(|| async {
                loop {
                    thread::sleep(Duration::from_secs(1));
                    RequestCounterService::get_instance().write().await.clean();
                }
            });

            RwLock::new(RequestCounterService::default())
        })
    }
}
