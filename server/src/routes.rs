use crate::discovery::DiscoveryRequest;
use rocket::form::Form;
use std::net::SocketAddr;

#[get("/")]
fn home() -> String {
    "Hello Jabacat!".to_string()
}

#[post("/discover", data = "<discoveryrequest>")]
fn discover(remote_addr: SocketAddr, mut discoveryrequest: Form<DiscoveryRequest>) -> String {
    if discoveryrequest.ip.is_none() {
        discoveryrequest.ip = Some(remote_addr.ip());
    }
    debug!("{:?}", discoveryrequest);
    "Looking for clients".to_string()
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![home, discover]
}
