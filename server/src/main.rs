#[macro_use]
extern crate rocket;

pub mod cleanable;
pub mod discovery;
pub mod routes;

use common::structures;
use routes::get_routes;
use std::collections::HashMap;
use std::sync::RwLock;

#[launch]
pub fn rocket() -> _ {
    // Normal env_logger::init will crash when rocket client is run in two different tests
    match env_logger::try_init() {
        Ok(..) => debug!("Logger started!"),
        Err(_) => debug!("Logger already exists!"),
    }

    rocket::build()
        .mount("/", get_routes())
        .manage(discovery::DiscoveryQueue {
            queue: RwLock::new(HashMap::new()),
        })
}
