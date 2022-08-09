use crate::discovery::DiscoveryRequest;
use std::net::SocketAddr;
use rocket::form::Form;

#[get("/")]
fn home() -> String {
    "Hello Jabacat!".to_string()
}

#[post("/discover", data = "<discoveryrequest>")]
fn discover(remote_addr: SocketAddr, discoveryrequest: Form<DiscoveryRequest>) -> String {
    println!("{:?}", discoveryrequest);
    if discoveryrequest.ip.is_none() {
        return remote_addr.to_string();
    }
    "Looking for clients".to_string()
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![home, discover]
}
