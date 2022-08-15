use rocket::{http::Status, serde::json::Json};

use crate::{
    guard::request_count,
    server::node::{NodeInfo, NodeInfos, NodeInfosTrait},
    utils::instance::OakSingleton,
};
#[get("/", format = "json")]
pub async fn get_infos(request_count: request_count::RequestCountGuard) -> Option<Json<NodeInfos>> {
    let x = NodeInfos::get_instance()
        .read()
        .await
        .get_node_stats()
        .clone();
    Some(Json(x))
}

#[get("/<node_name>", format = "json")]
pub async fn get_node_info(
    node_name: String,
    request_count: request_count::RequestCountGuard,
) -> Option<Json<NodeInfo>> {
    let x = NodeInfos::get_instance()
        .read()
        .await
        .get_node_stats()
        .drain_filter(|v| v.name == node_name)
        .collect::<NodeInfos>();

    let x = x.first();

    match x {
        Some(o) => Some(Json(o.clone())),
        None => None,
    }
}

#[post("/<node_name>", format = "json", data = "<node_info>")]
pub async fn push_node_info(
    node_name: String,
    node_info: Json<NodeInfo>,
    request_count: request_count::RequestCountGuard,
) -> Status {
    if node_name != node_info.name {
        Status::BadRequest
    } else {
        NodeInfos::get_instance()
            .write()
            .await
            .set_node_info(node_info.0);
        Status::Accepted
    }
}
