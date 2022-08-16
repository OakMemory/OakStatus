use rocket::serde::json::Json;

use crate::{
    guard::{request_count, secret_vertify},
    server::world::{WorldInfo, Worlds, WorldsTrait},
    utils::instance::OakSingleton,
};

#[get("/", format = "json")]
pub async fn get_worlds(request_count: request_count::RequestCountGuard) -> Json<Worlds> {
    Json(Worlds::get_instance().read().await.clone())
}

#[get("/<uuid>", format = "json")]
pub async fn get_world_info(
    uuid: String,
    request_count: request_count::RequestCountGuard,
) -> Option<Json<WorldInfo>> {
    match Worlds::get_instance().read().await.get_world(uuid) {
        Some(o) => Some(Json(o)),
        None => None,
    }
}

#[post("/", format = "json", data = "<world_infos>")]
pub async fn push_worlds(
    world_infos: Json<Vec<WorldInfo>>,
    request_count: request_count::RequestCountGuard,
    secret_vertify: secret_vertify::SecretVertifyGuard,
) {
    for ele in world_infos.iter() {
        Worlds::get_instance().write().await.put_world(ele.clone());
    }
}

#[post("/<uuid>", format = "json", data = "<world_info>")]
pub async fn push_world_info(
    uuid: String,
    world_info: Json<WorldInfo>,
    request_count: request_count::RequestCountGuard,
    secret_vertify: secret_vertify::SecretVertifyGuard,
) {
    Worlds::get_instance().write().await.put_world(world_info.0);
}
