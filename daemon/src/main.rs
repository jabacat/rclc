use reqwest;

use client_server::client_daemon::{
    client_daemon_server::ClientDaemonServer, event, Event, Message,
};
use common::structures::{DiscoveryRequest, DiscoveryResponse, InfoResponse};
use discovery::{discover, discover_info, discover_root, DiscoveryServerConfig};
use std::net::{IpAddr, Ipv4Addr};
use tonic::transport::Server;

pub mod client_server;
pub mod contact;
pub mod discovery;
pub mod notif;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    println!("Hello, world!");

    //notif::notif("RCLC", "The RCLC daemon has been launched!");

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

    // tonic gRPC server for the communication with the client
    let addr = "0.0.0.0:5768".parse()?;
    let (ctx, _crx) = tokio::sync::broadcast::channel(4);
    let cd_svc = client_server::ClientDaemonService {
        ev_stream: ctx.clone(),
    };
    // FIXME: Remove this when event stream is implemented, this is only for testing
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        loop {
            interval.tick().await;
            ctx.send(Ok(Event {
                event: Some(event::Event::Message(event::MessageEvent {
                    edit: false,
                    sender: 1,
                    message: Some(Message {
                        id: 1,
                        content: "Hello".to_string(),
                        user: 1,
                        timestamp: 1,
                    }),
                })),
            }))
            .unwrap();
        }
    });
    let cd_srv = ClientDaemonServer::new(cd_svc);
    println!("Client gRPC server on {}", addr);
    Server::builder().add_service(cd_srv).serve(addr).await?;

    Ok(())
}
