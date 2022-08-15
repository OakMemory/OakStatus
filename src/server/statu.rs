use std::time::SystemTime;

use once_cell::sync::OnceCell;
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::utils::instance::OakSingleton;

pub type Statu = Vec<StatuInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuInfo {
    node_name: String,
    last_push: SystemTime,
    is_down: bool,
}

pub trait StatusTrait {
    fn get_statu(&self, node_name: String) -> Option<StatuInfo>;
    fn get_status(&self) -> Statu;
    fn set_down(&mut self, node_name: String);
    fn update(&mut self, node_name: String);
}

impl StatusTrait for Statu {
    fn get_statu(&self, node_name: String) -> Option<StatuInfo> {
        for ele in self {
            if ele.node_name == node_name {
                return Some(ele.clone());
            }
        }
        None
    }

    fn get_status(&self) -> Statu {
        self.clone()
    }

    fn set_down(&mut self, node_name: String) {
        for (index, ele) in self.iter().enumerate() {
            if ele.node_name == node_name {
                let ele = StatuInfo {
                    is_down: true,
                    ..ele.clone()
                };

                self.push(ele);
                self.swap_remove(index);
                break;
            }
        }
    }

    fn update(&mut self, node_name: String) {
        for (index, ele) in self.iter().enumerate() {
            if ele.node_name == node_name {
                let ele = StatuInfo {
                    last_push: SystemTime::now(),
                    is_down: false,
                    ..ele.clone()
                };
                self.push(ele);
                self.swap_remove(index);
                break;
            }
        }
    }
}

impl OakSingleton for Statu {
    fn get_instance() -> &'static RwLock<Statu> {
        static INSTANCE: OnceCell<RwLock<Statu>> = OnceCell::new();
        INSTANCE.get_or_init(|| RwLock::new(Statu::default()))
    }
}
