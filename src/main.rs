#![feature(drain_filter)]

mod api;
mod config;
mod guard;
mod middleware;
mod server;
mod service;
mod utils;

use crate::api::node::{get_info, get_infos, push_cpu_load, push_info, push_memory_load};
use crate::api::player::{get_player, get_players, push_player, push_players};
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
            routes![
                get_infos,
                push_info,
                get_info,
                push_cpu_load,
                push_memory_load,
            ],
        )
        .mount(
            "/api/player",
            routes![get_player, get_players, push_player, push_players],
        )
        .mount(
            "/api/world",
            routes![get_world_info, get_worlds, push_world_info, push_worlds],
        )
}
