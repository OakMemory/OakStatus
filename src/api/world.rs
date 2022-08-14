use rocket::serde::json::Json;

use crate::{
    instance::OakSingleton,
    server::world::{WorldInfo, Worlds, WorldsTrait},
};

#[get("/")]
pub fn get_worlds() {}

#[get("/<uuid>")]
pub async fn get_world_info(uuid: String) -> Option<Json<WorldInfo>> {
    match Worlds::get_instance().lock().await.get_world(uuid) {
        Some(o) => Some(Json(o)),
        None => None,
    }
}

#[post("/", format = "json", data = "<world_infos>")]
pub async fn push_worlds(world_infos: Json<Vec<WorldInfo>>) {
    for ele in world_infos.iter() {
        Worlds::get_instance().lock().await.put_world(ele.clone());
    }
}

#[post("/<uuid>", format = "json", data = "<world_info>")]
pub async fn push_world_info(uuid: String, world_info: Json<WorldInfo>) {
    Worlds::get_instance().lock().await.put_world(world_info.0);
}
