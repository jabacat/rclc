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

/// Parses a Vec<u8> into a ClientToDaemonMsg object
pub fn parse_message_client_to_daemon(
    data: Vec<u8>,
) -> Result<ClientToDaemonMsg, PacketParseError> {
    let out: ClientToDaemonMsg = postcard::from_bytes(&data).map_err(|_| PacketParseError)?;
    Ok(out)
}

/// Converts a DaemonToClientMsg object into bytes to be sent over a stream
pub fn serialize_message_daemon_to_client(
    message: DaemonToClientMsg,
) -> Result<Vec<u8>, postcard::Error> {
    //let out: heapless::Vec<u8, 128> = postcard::to_vec(&message)?;
    let out: Vec<u8> = postcard::to_vec::<DaemonToClientMsg, 16384>(&message)?.to_vec();
    // TODO: The number 16384 is the amount of bytes that will be read, if it exceeds this number it will
    // error. I'm not yet sure how to make this dyanmically alocate so I just set it to a really
    // high multiple of 2.
    Ok(out.to_vec())
}

/// Converts a ClientToDaemon object into bytes to be sent over a stream
pub fn serialize_message_client_to_daemon(
    message: ClientToDaemonMsg,
) -> Result<Vec<u8>, postcard::Error> {
    //let out: heapless::Vec<u8, 128> = postcard::to_vec(&message)?;
    let out: Vec<u8> = postcard::to_vec::<ClientToDaemonMsg, 16384>(&message)?.to_vec();
    // TODO: The number 16384 is the amount of bytes that will be read, if it exceeds this number it will
    // error. I'm not yet sure how to make this dyanmically alocate so I just set it to a really
    // high multiple of 2.
    Ok(out.to_vec())
}
