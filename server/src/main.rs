#[macro_use]
extern crate rocket;

pub mod discovery;
pub mod routes;

use common::structures;
use routes::get_routes;
use std::collections::HashMap;
use std::sync::RwLock;

#[launch]
pub fn rocket() -> _ {
    env_logger::init();
    rocket::build()
        .mount("/", get_routes())
        .manage(discovery::DiscoveryQueue {
            queue: RwLock::new(HashMap::new()),
        })
}
