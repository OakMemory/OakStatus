use rocket::{http::Status, serde::json::Json};

use crate::server::{self, NodeInfo, NodeInfos};

#[get("/", format = "json")]
pub async fn get_infos() -> Option<Json<NodeInfos>> {
    let x = server::get_node_stats().await.clone();
    Some(Json(x))
}

#[get("/<node_name>", format = "json")]
pub async fn get_node_info(node_name: String) -> Option<Json<NodeInfo>> {
    let x = server::get_node_stats()
        .await
        .drain_filter(|v| v.node_name == node_name)
        .collect::<NodeInfos>();

    let x = x.first();

    match x {
        Some(o) => Some(Json(o.clone())),
        None => None,
    }
}

#[post("/<node_name>", format = "json", data = "<node_info>")]
pub async fn push_node_info(node_name: String, node_info: Json<NodeInfo>) -> Status {
    if node_name != node_info.node_name {
        Status::BadRequest
    } else {
        server::set_node_info(node_info.0).await;
        Status::Accepted
    }
}
