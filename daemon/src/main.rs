use reqwest;

use common::structures::DiscoveryRequest;
use discovery::{discover, discover_root, DiscoveryServerConfig};
use std::net::{IpAddr, Ipv4Addr};

pub mod discovery;
pub mod notif;

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Hello, world!");

    notif::notif("RCLC", "The RCLC daemon has been launched!");

    let disc_conf = DiscoveryServerConfig {
        url: "http://127.0.0.1:8000".to_string(),
    };

    match discover_root(disc_conf.clone()).await {
        Ok(a) => {
            println!("{a}")
        }
        Err(_) => eprintln!("Could not connect to server. Possible it does not exist yet."),
    }

    let disc_req_1 = DiscoveryRequest {
        ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        port: 2121,
        requested_by: "4857689".to_string(),
        looking_for: "389475783".to_string(),
        public_key: "qwertyuiop".to_string(),
    };

    match discover(disc_conf.clone(), disc_req_1).await {
        Ok(a) => {
            if a.contains("NoMatch") {
                println!("No match!");
            }
        }
        Err(_) => eprintln!("Could not connect to server. Possible it does not exist yet."),
    }

    let disc_req_2 = DiscoveryRequest {
        ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        port: 2121,
        requested_by: "389475783".to_string(),
        looking_for: "4857689".to_string(),
        public_key: "qwertyuiop".to_string(),
    };

    match discover(disc_conf, disc_req_2).await {
        Ok(a) => {
            if !a.contains("NoMatch") {
                println!("Match!");
            }
        }
        Err(_) => eprintln!("Could not connect to server. Possible it does not exist yet."),
    }
}
