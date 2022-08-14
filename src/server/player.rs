use once_cell::sync::OnceCell;
use rocket::tokio::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::utils::instance::OakSingleton;

pub type Players = Vec<PlayerInfo>;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct PlayerInfo {
    name: String,
    life: String,
    world: String,
    server: String,
    is_online: bool,
}

pub trait PlayersTrait {
    fn put_players(&mut self, players: Vec<PlayerInfo>);
    fn put_player(&mut self, player_info: PlayerInfo);
    fn get_player(&self, player_name: String) -> Option<PlayerInfo>;
}

impl PlayersTrait for Players {
    fn put_players(&mut self, players: Vec<PlayerInfo>) {
        for ele in players {
            self.put_player(ele)
        }
    }

    fn put_player(&mut self, player_info: PlayerInfo) {
        for (index, value) in self.clone().iter().enumerate() {
            if value.name == player_info.name {
                self.remove(index);
                self.push(player_info);
                return;
            }
        }
        self.push(player_info);
    }

    fn get_player(&self, player_name: String) -> Option<PlayerInfo> {
        for ele in self {
            if ele.name == player_name {
                Some(ele.clone());
            }
        }
        None
    }
}

impl OakSingleton for Players {
    fn get_instance() -> &'static Mutex<Players> {
        static INSTANCE: OnceCell<Mutex<Players>> = OnceCell::new();
        INSTANCE.get_or_init(|| Mutex::new(Players::default()))
    }
}
