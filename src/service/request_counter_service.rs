use std::{sync::Mutex, thread, time::Duration};

use once_cell::sync::OnceCell;

use super::OakService;

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

impl OakService for RequestCounterService {
    fn get_instance() -> &'static Mutex<RequestCounterService> {
        static INSTANCE: OnceCell<Mutex<RequestCounterService>> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            thread::spawn(|| loop {
                thread::sleep(Duration::from_secs(1));
                let x = RequestCounterService::get_instance().lock();
                x.unwrap().clean();
            });

            Mutex::new(RequestCounterService::default())
        })
    }
}
