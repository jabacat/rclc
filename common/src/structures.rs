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
