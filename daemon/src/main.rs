use reqwest;

use common::structures;
use discovery::{discover_root, DiscoveryServerConfig};

pub mod discovery;
pub mod notif;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    notif::notif("RCLC", "The RCLC daemon has been launched!");

    let disc_conf = DiscoveryServerConfig {
        url: "http://127.0.0.1:8000".to_string(),
    };

    match discover_root(disc_conf).await {
        Ok(a) => {
            println!("{a}")
        }
        Err(_) => eprintln!("Could not connect to server. Possible it does not exist yet."),
    }
}
