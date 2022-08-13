#![feature(drain_filter)]

mod api;
mod guard;
mod middleware;
mod server;
mod service;
use crate::api::server_stats::{get_infos, get_node_info, push_node_info};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/api/server",
        routes![get_infos, push_node_info, get_node_info],
    )
}
