use crate::discovery::{Advertisement, DiscoveryQueue};
use rocket::serde::json::Json;
use rocket::State;
use std::net::SocketAddr;
use std::time::SystemTime;
use crate::structures::{DiscoveryResponse, Status, DiscoveryRequest};

#[get("/")]
fn home() -> String {
    "Hello Jabacat!".to_string()
}

#[get("/info")]
fn info() -> String {
    format!(
        r#"{{"motd":"{}","version":"{}","acceptingrequests":true}}"#,
        option_env!("RCLC_DISCOVERY_MOTD")
            .unwrap_or("Set an MOTD with the RCLC_DISCOVERY_MOTD environment variable"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
    )
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
    discoveryqueue
        .queue
        .write()
        .expect("Failed to gain lock on discovery queue")
        .insert(discoveryrequest.requested_by.clone(), advert);

    match discoveryqueue
        .queue
        .read()
        .expect("Failed to gain lock on discovery queue")
        .get(&discoveryrequest.looking_for)
    {
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
        }
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
    routes![home, discover, info]
}

#[cfg(test)]
mod test {
    use super::super::rocket;
    use rocket::http::ContentType;
    use rocket::http::Status as HttpStatus;
    use rocket::local::blocking::Client;

    #[test]
    fn discover_test() {
        let client = Client::tracked(rocket()).unwrap();
        let response_a = client.post("/discover").header(ContentType::JSON).body(r#"{"ip":"123.123.123.123","port":1000,"requested_by":"PersonA","looking_for":"PersonB","public_key":"abcdefg1"}"#).dispatch();
        let response_b = client.post("/discover").header(ContentType::JSON).body(r#"{"ip":"123.123.123.123","port":1000,"requested_by":"PersonB","looking_for":"PersonA","public_key":"abcdefg2"}"#).dispatch();
        assert_eq!(response_a.status(), HttpStatus::Ok);
        assert_eq!(response_b.status(), HttpStatus::Ok);
        assert_eq!(
            response_a.into_string().unwrap(),
            r#"{"status":"NoMatch","error":null,"discovery":null,"message":"No client found, advertisement placed"}"#
        );
        assert_eq!(
            response_b.into_string().unwrap(),
            r#"{"status":"Match","error":null,"discovery":{"ip":"123.123.123.123","port":1000,"requested_by":"PersonA","looking_for":"PersonB","public_key":"abcdefg1"},"message":"It's a match!"}"#
        );
    }

    #[test]
    fn info_test() {
        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/info").dispatch();
        assert_eq!(response.status(), HttpStatus::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            format!(
                r#"{{"motd":"{}","version":"{}","acceptingrequests":true}}"#,
                option_env!("RCLC_DISCOVERY_MOTD")
                    .unwrap_or("Set an MOTD with the RCLC_DISCOVERY_MOTD environment variable"),
                option_env!("CARGO_PKG_VERSION").unwrap_or("unknown")
            )
        );
    }
}
