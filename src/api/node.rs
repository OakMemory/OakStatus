use rocket::serde::json::Json;

use crate::{
    config::OakConfig,
    guard::{request_count, secret_vertify},
    server::node::{NodeInfo, NodeInfos, NodeInfosTrait},
    utils::instance::OakSingleton,
};
#[get("/", format = "json")]
pub async fn get_infos(request_count: request_count::RequestCountGuard) -> Option<Json<NodeInfos>> {
    let x = NodeInfos::get_instance().read().await.clone();
    Some(Json(x))
}

#[get("/<node_name>", format = "json")]
pub async fn get_info(
    node_name: String,
    request_count: request_count::RequestCountGuard,
) -> Option<Json<NodeInfo>> {
    match NodeInfos::get_instance()
        .read()
        .await
        .get(&node_name)
        .cloned()
    {
        Some(o) => Some(Json(o.clone())),
        None => None,
    }
}

#[post("/<node_name>", format = "json", data = "<node_info>")]
pub async fn push_info(
    node_name: String,
    node_info: Json<NodeInfo>,
    request_count: request_count::RequestCountGuard,
    token_vertify: secret_vertify::SecretVertifyGuard,
) {
    match NodeInfos::get_instance().read().await.get(&node_name) {
        Some(_) => {
            NodeInfos::get_instance()
                .write()
                .await
                .set_node_info(node_name, node_info.0);
        }
        None => return,
    }
}

#[post("/<node_name>/cpu_load", format = "json", data = "<cpu_load>")]
pub async fn push_cpu_load(
    node_name: String,
    cpu_load: Json<Vec<f32>>,
    request_count: request_count::RequestCountGuard,
    token_vertify: secret_vertify::SecretVertifyGuard,
) {
    let node_name = Box::new(node_name);

    let c = OakConfig::get_instance().read();
    NodeInfos::get_instance()
        .write()
        .await
        .append_cpu_load(*node_name.clone(), cpu_load.0.clone());

    let c = c.await.extract_inner("storage_time").unwrap();

    NodeInfos::get_instance()
        .write()
        .await
        .cut_memory_load(*node_name.clone(), c);
}

#[post("/<node_name>/memory_load", format = "json", data = "<memory_load>")]
pub async fn push_memory_load(
    node_name: String,
    memory_load: Json<Vec<f32>>,
    request_count: request_count::RequestCountGuard,
    token_vertify: secret_vertify::SecretVertifyGuard,
) {
    let node_name = Box::new(node_name);

    let c = OakConfig::get_instance().read();
    NodeInfos::get_instance()
        .write()
        .await
        .append_memory_load(*node_name.clone(), memory_load.0.clone());

    let c = c.await.extract_inner("storage_time").unwrap();

    NodeInfos::get_instance()
        .write()
        .await
        .cut_memory_load(*node_name.clone(), c);
}
