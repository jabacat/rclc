#[cfg(feature = "rocket")]
use rocket::FromForm;

use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "rocket", derive(FromForm))]
pub struct DiscoveryRequest {
    pub ip: Option<IpAddr>,
    pub port: u16,
    pub requested_by: String,
    pub looking_for: String,
    pub public_key: String, // Lets get proper parsing on this value done at some point
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub enum Status {
    Match,
    NoMatch,
    Failure,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct DiscoveryResponse {
    pub status: Status,
    pub error: Option<String>,
    pub discovery: Option<DiscoveryRequest>,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoResponse {
    pub motd: String,
    pub version: String,
    pub acceptingrequests: bool,
}
