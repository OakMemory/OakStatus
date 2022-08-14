use rocket::serde::json::Json;

use crate::{
    guard::request_count,
    server::player::{PlayerInfo, Players, PlayersTrait},
    utils::instance::OakSingleton,
};

#[get("/")]
pub async fn get_players(request_count: request_count::RequestCountGuard) -> Option<Json<Players>> {
    Some(Json(Players::get_instance().lock().await.clone()))
}

#[get("/<player_name>")]
pub async fn get_player(
    player_name: String,
    request_count: request_count::RequestCountGuard,
) -> Option<Json<PlayerInfo>> {
    match Players::get_instance().lock().await.get_player(player_name) {
        Some(o) => Some(Json(o)),
        None => None,
    }
}

#[post("/", format = "json", data = "<players>")]
pub async fn push_players(players: Json<Players>, request_count: request_count::RequestCountGuard) {
    Players::get_instance().lock().await.put_players(players.0);
}

#[post("/<player_name>", format = "json", data = "<player_info>")]
pub async fn push_player(
    player_name: String,
    player_info: Json<PlayerInfo>,
    request_count: request_count::RequestCountGuard,
) {
    Players::get_instance()
        .lock()
        .await
        .put_player(player_info.0);
}
