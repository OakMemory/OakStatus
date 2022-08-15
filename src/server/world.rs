use once_cell::sync::OnceCell;
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::utils::instance::OakSingleton;

pub type Worlds = Vec<WorldInfo>;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct WorldInfo {
    player_count: i8,
    world_uuid: String,
    world_size: i32,
    world_name: String,
}

pub trait WorldsTrait {
    fn put_world(&mut self, world_info: WorldInfo) -> bool;
    fn put_worlds(&mut self, world_infos: Vec<WorldInfo>);
    fn get_world(&self, uuid: String) -> Option<WorldInfo>;
}

impl WorldsTrait for Worlds {
    fn put_world(&mut self, world_info: WorldInfo) -> bool {
        for (index, value) in self.iter().enumerate() {
            if value.world_uuid == world_info.world_uuid {
                self.remove(index);
                self.push(world_info);

                return true;
            }
        }
        self.push(world_info);
        true
    }

    fn put_worlds(&mut self, world_infos: Vec<WorldInfo>) {
        for ele in world_infos {
            self.put_world(ele);
        }
    }

    fn get_world(&self, uuid: String) -> Option<WorldInfo> {
        for ele in self {
            if ele.world_uuid == uuid {
                return Some(ele.clone());
            }
        }
        None
    }
}

impl OakSingleton for Worlds {
    fn get_instance() -> &'static RwLock<Worlds> {
        static INSTANCE: OnceCell<RwLock<Worlds>> = OnceCell::new();
        INSTANCE.get_or_init(|| RwLock::new(Worlds::default()))
    }
}
