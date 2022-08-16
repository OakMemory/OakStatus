use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request,
};

use crate::{service::request_counter::RequestCounterService, utils::instance::OakSingleton};

pub struct RequestCounter;
#[rocket::async_trait]
impl Fairing for RequestCounter {
    fn info(&self) -> Info {
        Info {
            name: "Request Counter",
            kind: Kind::Request | Kind::Singleton,
        }
    }

    async fn on_request(&self, _request: &mut Request<'_>, _: &mut Data<'_>) {
        RequestCounterService::get_instance()
            .write()
            .await
            .increase();
    }
}
