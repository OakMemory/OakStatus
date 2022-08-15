use std::collections::VecDeque;

use once_cell::sync::OnceCell;
use rocket::{
    serde::{Deserialize, Serialize},
    tokio::sync::RwLock,
};

use crate::utils::instance::OakSingleton;

pub type NodeInfos = Vec<NodeInfo>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct NodeInfo {
    pub name: String,
    pub usage_cpu: VecDeque<f32>,
    pub usage_memory: VecDeque<f32>,
}

pub trait NodeInfosTrait {
    fn set_node_info(&mut self, node_info: NodeInfo);
    fn get_node_stats(&self) -> Self;
}

impl NodeInfosTrait for NodeInfos {
    fn get_node_stats(&self) -> Self {
        self.clone()
    }

    fn set_node_info(&mut self, node_info: NodeInfo) {
        let node_info = Box::new(node_info.clone());

        let x = self.get_node_stats();

        // 刷新完毕的状态列表
        let mut t: NodeInfos = vec![];
        let mut fetched = false;
        for value in x.iter() {
            if value.name.eq(&node_info.name) {
                t.push(*node_info.clone());
                fetched = true;
            } else {
                t.push(value.clone())
            }
        }

        if fetched == false {
            t.push(*node_info.clone())
        }

        self.clear();
        self.append(&mut t)
    }
}

impl OakSingleton for NodeInfos {
    fn get_instance() -> &'static RwLock<NodeInfos> {
        static INSTANCE: OnceCell<RwLock<NodeInfos>> = OnceCell::new();
        INSTANCE.get_or_init(|| RwLock::new(NodeInfos::default()))
    }
}
