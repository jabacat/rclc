use reqwest;

use common::notif::notif;
use common::structures::{DiscoveryRequest, DiscoveryResponse, InfoResponse};
use discovery::{discover, discover_info, discover_root, DiscoveryServerConfig};
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

pub mod contact;
pub mod discovery;
pub mod listen;

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Hello, world!");

    notif("RCLC", "The RCLC daemon has been launched!");

    let disc_conf = DiscoveryServerConfig {
        url: "http://127.0.0.1:8000".to_string(),
    };

    // Request the discovery server root
    match discover_root(disc_conf.clone()).await {
        Ok(a) => {
            println!("{a}")
        }
        Err(_) => eprintln!("Could not connect to server. Possible it does not exist yet."),
    }

    // Request the discovery server info
    match discover_info(disc_conf.clone()).await {
        Ok(a) => {
            println!("{:?}", a)
        }
        Err(_) => eprintln!("Could not connect to server. Possible it does not exist yet."),
    }

    let disc_req_a = DiscoveryRequest {
        ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        port: 2121,
        requested_by: "A".to_string(),
        looking_for: "B".to_string(),
        public_key: "qwertyuiop".to_string(),
    };

    // Request to connect as person A
    // Match is not found because A is first
    match discover(disc_conf.clone(), disc_req_a).await {
        Ok(a) => {
            println!("{:?}", a);
        }
        Err(_) => eprintln!("Could not connect to server. Possible it does not exist yet."),
    }

    let disc_req_b = DiscoveryRequest {
        ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
        port: 2121,
        requested_by: "B".to_string(),
        looking_for: "A".to_string(),
        public_key: "qwertyuiop".to_string(),
    };

    // Request to connect as person B
    // Match is found because B is the second
    match discover(disc_conf, disc_req_b).await {
        Ok(a) => {
            println!("{:?}", a);
        }
        Err(_) => eprintln!("Could not connect to server. Possible it does not exist yet."),
    }

    // Try and listen from packets from the client
    listen::listen("/tmp/rclc.sock");

    loop {
        std::thread::sleep(Duration::from_millis(500));
    }
}
