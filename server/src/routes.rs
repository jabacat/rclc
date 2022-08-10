use crate::discovery::DiscoveryRequest;
use rocket::serde::json::Json;
use std::net::SocketAddr;

#[get("/")]
fn home() -> String {
    "Hello Jabacat!".to_string()
}

#[post("/discover", format = "json", data = "<discoveryrequest>")]
// If the ip is not provided, use the ip of the client sending the request
fn discover(
    remote_addr: Option<SocketAddr>,
    mut discoveryrequest: Json<DiscoveryRequest>,
) -> String {
    if discoveryrequest.ip.is_none() {
        let ip = match remote_addr {
            Some(addr) => Some(addr.ip()),
            None => None,
        };
        discoveryrequest.ip = ip;
    }
    debug!("{:?}", discoveryrequest);
    "Looking for clients".to_string()
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![home, discover]
}

#[cfg(test)]
mod test {
    use super::super::rocket;
    use rocket::http::ContentType;
    use rocket::local::blocking::Client;

    #[test]
    fn discover_test() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.post("/discover").header(ContentType::JSON).body(r#"{"ip":"93.184.216.34","port":1000,"requested_by":1408191,"looking_for":49384,"public_key":"545435454545"}"#).dispatch();
        assert_eq!(response.into_string().unwrap(), "Looking for clients");
    }
}
