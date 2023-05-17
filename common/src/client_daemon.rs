use chrono::{offset::Utc, DateTime};
use postcard::Error;
use serde::{Deserialize, Serialize};

use std::{
    fmt::{Display, Formatter},
    net::IpAddr,
    num::ParseIntError,
    str::ParseBoolError,
};

#[derive(Debug)]
pub struct PacketParseError;

impl Display for PacketParseError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Packet parse error occured")
    }
}

impl From<ParseIntError> for PacketParseError {
    fn from(_err: ParseIntError) -> Self {
        PacketParseError
    }
}

impl From<Error> for PacketParseError {
    fn from(_err: postcard::Error) -> Self {
        PacketParseError
    }
}

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
    Test(String),
    Connect,
    Disconnect,
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum DaemonToClientMsg {
    Recieved(Message),
    Test(String),
    Unknown,
}

pub fn parse_message(data: Vec<u8>) -> Result<ClientToDaemonMsg, PacketParseError> {
    let out: ClientToDaemonMsg = postcard::from_bytes(&data).map_err(|_| PacketParseError)?;
    Ok(out)
}

pub fn serialize_message_daemon_to_client(
    message: DaemonToClientMsg,
) -> Result<Vec<u8>, postcard::Error> {
    //let out: heapless::Vec<u8, 128> = postcard::to_vec(&message)?;
    let out: Vec<u8> = postcard::to_vec::<DaemonToClientMsg, 16384>(&message)?.to_vec();
    Ok(out.to_vec())
}

pub fn serialize_message_client_to_daemon(
    message: ClientToDaemonMsg,
) -> Result<Vec<u8>, postcard::Error> {
    //let out: heapless::Vec<u8, 128> = postcard::to_vec(&message)?;
    let out: Vec<u8> = postcard::to_vec::<ClientToDaemonMsg, 16384>(&message)?.to_vec();
    Ok(out.to_vec())
}
