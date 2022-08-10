use chrono::{DateTime, offset::Utc};
use serde::{Serialize, Deserialize};

use std::net::IpAddr;

/// a literal message sent from one peer
#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub content: String,
    pub time: DateTime<Utc>,
    pub origin: IpAddr,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ClientToDaemonMsg {
    Block(IpAddr),
    Send(String),
    Connect,
    Disconnect,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum DaemonToClientMsg {
    Recieved(Message),
}
