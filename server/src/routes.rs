use crate::discovery::{DiscoveryRequest, DiscoveryQueue, Advertisement, DiscoveryResponse, Status};
use rocket::serde::json::Json;
use rocket::State;
use std::net::SocketAddr;
use std::time::SystemTime;

#[get("/")]
fn home() -> String {
    "Hello Jabacat!".to_string()
}

#[post("/discover", format = "json", data = "<discoveryrequest>")]
// If the ip is not provided, use the ip of the client sending the request
fn discover(
    remote_addr: Option<SocketAddr>,
    mut discoveryrequest: Json<DiscoveryRequest>,
    discoveryqueue: &State<DiscoveryQueue>,
) -> Json<DiscoveryResponse> {
    if discoveryrequest.ip.is_none() {
        let ip = match remote_addr {
            Some(addr) => Some(addr.ip()),
            None => None,
        };
        discoveryrequest.ip = ip;
    }

    // Place the users request in the queue
    let advert = Advertisement {
        discovery: discoveryrequest.clone().into_inner(),
        created_at: SystemTime::now(),
        expires_in: 5000,
    };
    println!("{:?}", &advert);
    discoveryqueue.queue.write().expect("Failed to gain lock on discovery queue").insert(discoveryrequest.requested_by.clone(), advert);


    match discoveryqueue.queue.read().expect("Failed to gain lock on discovery queue").get(&discoveryrequest.looking_for) {
        Some(a) => {
            if &discoveryrequest.requested_by == &a.discovery.looking_for {
                println!("It's a match! {:?}", a);
                return Json(DiscoveryResponse {
                    status: Status::Match,
                    discovery: Some(a.discovery.clone()),
                    error: None,
                    message: "It's a match!".to_string(),
                });
            } else {
                println!("No match! {:?}", a);
                return Json(DiscoveryResponse {
                    status: Status::NoMatch,
                    discovery: None,
                    error: None,
                    message: "No client found, advertisement placed".to_string(),
                });
            }
        },
        None => {
            println!("No advertisement found");
            return Json(DiscoveryResponse {
                status: Status::NoMatch,
                discovery: None,
                error: None,
                message: "No client found, advertisement placed".to_string(),
            });
        }
    };
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
