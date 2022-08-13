use std::collections::VecDeque;

use once_cell::sync::OnceCell;
use rocket::{
    serde::{Deserialize, Serialize},
    tokio::sync::{Mutex, MutexGuard},
};

pub type NodeInfos = Vec<NodeInfo>;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "rocket::serde")]
pub struct NodeInfo {
    pub node_name: String,
    pub node_usage_cpu: VecDeque<f32>,
    pub node_usage_memory: VecDeque<f32>,
}

static mut _NODE_INFOS: OnceCell<Mutex<NodeInfos>> = OnceCell::new();

pub async fn get_node_stats() -> MutexGuard<'static, Vec<NodeInfo>> {
    unsafe {
        _NODE_INFOS
            .get_or_init(|| Mutex::new(Vec::new()))
            .lock()
            .await
    }
}

pub async fn set_node_info(node_info: NodeInfo) {
    let node_info = Box::new(node_info.clone());

    let x = get_node_stats();

    // 刷新完毕的状态列表
    let mut t: NodeInfos = vec![];
    let mut fetched = false;
    for value in x.await.iter() {
        if value.node_name.eq(&node_info.node_name) {
            t.push(*node_info.clone());
            fetched = true;
        } else {
            t.push(value.clone())
        }
    }

    if fetched == false {
        t.push(*node_info.clone())
    }

    unsafe {
        let mut locked = _NODE_INFOS.get_mut().unwrap().lock().await;
        locked.clear();
        locked.append(&mut t)
    }
}
