use rocket::serde::{Deserialize, Serialize};
use rocket::FromForm;
use std::net::IpAddr;

#[derive(FromForm, Debug, Deserialize, Serialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DiscoveryRequest {
    pub ip: Option<IpAddr>,
    pub port: u16,
    pub requested_by: String,
    pub looking_for: String,
    pub public_key: String, // Lets get proper parsing on this value done at some point
}
