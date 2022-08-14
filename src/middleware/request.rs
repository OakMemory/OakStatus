use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request,
};

use crate::{instance::OakSingleton, service::request_counter::RequestCounterService};

pub struct RequestCounter;
#[rocket::async_trait]
impl Fairing for RequestCounter {
    fn info(&self) -> Info {
        Info {
            name: "Request counter",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, _request: &mut Request<'_>, _: &mut Data<'_>) {
        RequestCounterService::get_instance()
            .lock()
            .await
            .increase();
    }
}
