use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};

use std::net::IpAddr;

/// a literal message sent from one peer
#[derive(Deserialize, Serialize)]
pub struct Message {
    content: String,
    time: DateTime<Utc>,
    origin: IpAddr,
}

#[derive(Deserialize, Serialize)]
pub enum ClientToDaemonMsg {
    Block(IpAddr),
    Send(String),
    Connect,
    Disconnect,
}

#[derive(Deserialize, Serialize)]
pub enum DaemonToClientMsg {
    Recieved(Message),
}
