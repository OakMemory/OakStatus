use rocket::serde::json::Json;

use crate::{
    guard::{request_count, secret_vertify},
    server::player::{PlayerInfo, Players, PlayersTrait},
    utils::instance::OakSingleton,
};

#[get("/", format = "json")]
pub async fn get_players(request_count: request_count::RequestCountGuard) -> Option<Json<Players>> {
    Some(Json(Players::get_instance().read().await.clone()))
}

#[get("/<player_name>", format = "json")]
pub async fn get_player(
    player_name: String,
    request_count: request_count::RequestCountGuard,
) -> Option<Json<PlayerInfo>> {
    match Players::get_instance().read().await.get_player(player_name) {
        Some(o) => Some(Json(o)),
        None => None,
    }
}

#[post("/", format = "json", data = "<players>")]
pub async fn push_players(
    players: Json<Players>,
    request_count: request_count::RequestCountGuard,
    token_vertify: secret_vertify::SecretVertifyGuard,
) {
    for (index, ele) in players.0.iter() {
        Players::get_instance()
            .write()
            .await
            .put_player(index.to_string(), ele.clone());
    }
}

#[post("/<player_name>", format = "json", data = "<player_info>")]
pub async fn push_player(
    player_name: String,
    player_info: Json<PlayerInfo>,
    request_count: request_count::RequestCountGuard,
    token_vertify: secret_vertify::SecretVertifyGuard,
) {
    Players::get_instance()
        .write()
        .await
        .put_player(player_name, player_info.0);
}
