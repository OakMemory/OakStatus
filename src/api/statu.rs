use rocket::serde::json::Json;

use crate::{
    guard::request_count,
    server::statu::{Statu, StatuInfo, StatusTrait},
    utils::instance::OakSingleton,
};

#[get("/<node_name>", format = "json")]
pub async fn get_statu(
    node_name: String,
    request_count: request_count::RequestCountGuard,
) -> Option<Json<StatuInfo>> {
    match Statu::get_instance().read().await.get_statu(node_name) {
        Some(o) => Some(Json(o)),
        None => None,
    }
}

#[get("/", format = "json")]
pub async fn get_status(request_count: request_count::RequestCountGuard,) -> Json<Statu> {
    Json(Statu::get_instance().read().await.clone())
}
