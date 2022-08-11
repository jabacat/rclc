#[macro_use]
extern crate rocket;

pub mod discovery;
pub mod routes;

use routes::get_routes;
use std::collections::HashMap;
use std::sync::RwLock;

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .mount("/", get_routes())
        .manage(discovery::DiscoveryQueue {
            queue: RwLock::new(HashMap::new()),
        })
}
