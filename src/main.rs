#![feature(drain_filter)]
#![feature(exclusive_range_pattern)]
mod api;
mod config;
mod guard;
mod middleware;
mod server;
mod service;
mod utils;

use crate::api::node::{get_infos, get_node_info, push_node_info};
use crate::api::player::{get_player, get_players, push_player, push_players};
use crate::api::statu::{get_statu, get_status};
use crate::api::world::{get_world_info, get_worlds, push_world_info, push_worlds};

use middleware::request::RequestCounter;
use utils::instance::OakSingleton;

#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::custom(config::OakConfig::get_instance().read().await.clone())
        .attach(RequestCounter)
        .mount(
            "/api/node",
            routes![get_infos, push_node_info, get_node_info],
        )
        .mount("/api/statu", routes![get_statu, get_status])
        .mount(
            "/api/player",
            routes![get_player, get_players, push_player, push_players],
        )
        .mount(
            "/api/world",
            routes![get_world_info, get_worlds, push_world_info, push_worlds],
        )
}
