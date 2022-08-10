#[macro_use]
extern crate rocket;

pub mod discovery;
pub mod routes;

use routes::get_routes;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", get_routes())
}
