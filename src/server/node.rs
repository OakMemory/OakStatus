use std::collections::HashMap;

use once_cell::sync::OnceCell;
use rocket::{
    serde::{Deserialize, Serialize},
    tokio::sync::RwLock,
};

use crate::utils::instance::OakSingleton;

pub type NodeInfos = HashMap<String, NodeInfo>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct NodeInfo {
    pub cpu_load: Vec<f32>,
    pub memory_load: Vec<f32>,
}

pub trait NodeInfosTrait {
    fn set_node_info(&mut self, node_name: String, node_info: NodeInfo);
    fn append_cpu_load(&mut self, name: String, append: Vec<f32>);
    fn append_memory_load(&mut self, name: String, append: Vec<f32>);
    fn cut_memory_load(&mut self, name: String, size: usize);
    fn cut_cpu_load(&mut self, name: String, size: usize);
}

impl NodeInfosTrait for NodeInfos {
    fn set_node_info(&mut self, node_name: String, node_info: NodeInfo) {
        self.insert(node_name, node_info);
    }

    fn append_cpu_load(&mut self, name: String, append: Vec<f32>) {
        // 获取信息
        let mut x = match self.get(&name) {
            Some(o) => o,
            None => return,
        }
        .clone();
        // 添加到最后
        x.cpu_load.copy_from_slice(&append);

        self.insert(name, x);
    }

    fn append_memory_load(&mut self, name: String, append: Vec<f32>) {
        // 获取信息
        let mut x = match self.get(&name) {
            Some(o) => o,
            None => return,
        }
        .clone();
        // 添加到最后
        x.memory_load.copy_from_slice(&append);

        self.insert(name, x);
    }

    fn cut_memory_load(&mut self, name: String, size: usize) {
        // 获取信息
        let mut x = match self.get(&name) {
            Some(o) => o,
            None => return,
        }
        .clone();
        // 裁剪
        if x.memory_load.len() >= size {
            let after_split = x.memory_load.split_at(x.memory_load.len() - size);
            x.memory_load = after_split.1.to_vec();
        }
    }

    fn cut_cpu_load(&mut self, name: String, size: usize) {
        // 获取信息
        let mut x = match self.get(&name) {
            Some(o) => o,
            None => return,
        }
        .clone();
        // 裁剪
        if x.cpu_load.len() >= size {
            let after_split = x.cpu_load.split_at(x.cpu_load.len() - size);
            x.cpu_load = after_split.1.to_vec();
        }
    }
}

impl OakSingleton for NodeInfos {
    fn get_instance() -> &'static RwLock<NodeInfos> {
        static INSTANCE: OnceCell<RwLock<NodeInfos>> = OnceCell::new();

        INSTANCE.get_or_init(|| RwLock::new(NodeInfos::default()))
    }
}
