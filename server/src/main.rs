#[macro_use]
extern crate rocket;

pub mod routes;
pub mod discovery;

use routes::get_routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", get_routes())
}
