use rocket::serde::json::Json;

use crate::{
    instance::OakSingleton,
    server::player::{PlayerInfo, Players, PlayersTrait},
};

#[get("/")]
pub async fn get_players() -> Option<Json<Players>> {
    Some(Json(Players::get_instance().lock().await.clone()))
}

#[get("/<player_name>")]
pub async fn get_player(player_name: String) -> Option<Json<PlayerInfo>> {
    match Players::get_instance().lock().await.get_player(player_name) {
        Some(o) => Some(Json(o)),
        None => None,
    }
}

#[post("/", format = "json", data = "<players>")]
pub async fn push_players(players: Json<Players>) {
    Players::get_instance().lock().await.put_players(players.0);
}

#[post("/<player_name>", format = "json", data = "<player_info>")]
pub async fn push_player(player_name: String, player_info: Json<PlayerInfo>) {
    Players::get_instance()
        .lock()
        .await
        .put_player(player_info.0);
}
