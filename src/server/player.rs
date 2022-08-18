use std::collections::HashMap;

use once_cell::sync::OnceCell;
use rocket::tokio::sync::RwLock;

use serde::{Deserialize, Serialize};

use crate::utils::instance::OakSingleton;

pub type Players = HashMap<String, PlayerInfo>;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    life: f32,
    world: String,
    server: String,
    is_online: bool,
}

pub trait PlayersTrait {
    fn put_player(&mut self, player_name: String, player_info: PlayerInfo);
    fn get_player(&self, player_name: String) -> Option<PlayerInfo>;
}

impl PlayersTrait for Players {
    fn put_player(&mut self, player_name: String, player_info: PlayerInfo) {
        self.insert(player_name, player_info);
    }

    fn get_player(&self, player_name: String) -> Option<PlayerInfo> {
        self.get(&player_name).cloned()
    }
}

impl OakSingleton for Players {
    fn get_instance() -> &'static RwLock<Players> {
        static INSTANCE: OnceCell<RwLock<Players>> = OnceCell::new();
        INSTANCE.get_or_init(|| RwLock::new(Players::default()))
    }
}
