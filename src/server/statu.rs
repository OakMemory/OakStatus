use std::{collections::HashMap, time::SystemTime};

use once_cell::sync::OnceCell;
use rocket::tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::utils::instance::OakSingleton;

pub type Statu = HashMap<String, StatuInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuInfo {
    last_push: SystemTime,
    is_online: bool,
}

pub trait StatusTrait {
    fn get_statu(&self, node_name: String) -> Option<StatuInfo>;
    fn get_status(&self) -> Statu;
    fn set_status(&mut self, node_name: String, is_online: bool);
    fn update_push(&mut self, node_name: String);
    fn add_node(&mut self, node_name: String);
}

impl StatusTrait for Statu {
    fn get_statu(&self, node_name: String) -> Option<StatuInfo> {
        for (name, info) in self {
            if *name == node_name {
                return Some(info.clone());
            }
        }
        None
    }

    fn get_status(&self) -> Statu {
        self.clone()
    }

    fn set_status(&mut self, node_name: String, is_online: bool) {
        match self.get(&node_name) {
            Some(o) => self.insert(
                node_name,
                StatuInfo {
                    is_online,
                    ..o.clone()
                },
            ),
            None => return,
        };
    }

    fn add_node(&mut self, node_name: String) {
        self.insert(
            node_name,
            StatuInfo {
                last_push: SystemTime::now(),
                is_online: true,
            },
        );
    }

    fn update_push(&mut self, node_name: String) {
        match self.get(&node_name) {
            Some(o) => self.insert(
                node_name,
                StatuInfo {
                    last_push: SystemTime::now(),
                    ..o.clone()
                },
            ),
            None => return,
        };
    }
}

impl OakSingleton for Statu {
    fn get_instance() -> &'static RwLock<Statu> {
        static INSTANCE: OnceCell<RwLock<Statu>> = OnceCell::new();
        INSTANCE.get_or_init(|| RwLock::new(Statu::default()))
    }
}
