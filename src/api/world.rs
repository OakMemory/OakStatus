use rocket::serde::json::Json;

use crate::{
    guard::request_count,
    server::world::{WorldInfo, Worlds, WorldsTrait},
    utils::instance::OakSingleton,
};

#[get("/")]
pub fn get_worlds() {}

#[get("/<uuid>")]
pub async fn get_world_info(
    uuid: String,
    request_count: request_count::RequestCountGuard,
) -> Option<Json<WorldInfo>> {
    match Worlds::get_instance().lock().await.get_world(uuid) {
        Some(o) => Some(Json(o)),
        None => None,
    }
}

#[post("/", format = "json", data = "<world_infos>")]
pub async fn push_worlds(
    world_infos: Json<Vec<WorldInfo>>,
    request_count: request_count::RequestCountGuard,
) {
    for ele in world_infos.iter() {
        Worlds::get_instance().lock().await.put_world(ele.clone());
    }
}

#[allow(unused_variables)]
#[post("/<uuid>", format = "json", data = "<world_info>")]
pub async fn push_world_info(
    uuid: String,
    world_info: Json<WorldInfo>,
    request_count: request_count::RequestCountGuard,
) {
    Worlds::get_instance().lock().await.put_world(world_info.0);
}
