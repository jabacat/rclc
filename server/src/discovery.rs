use rocket::serde::Deserialize;
use std::net::IpAddr;
use std::sync::RwLock;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(FromForm, Debug, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DiscoveryRequest {
    pub ip: Option<IpAddr>,
    pub port: u16,
    pub requested_by: String,
    pub looking_for: String,
    pub public_key: String, // Lets get proper parsing on this value done at some point
}

#[derive(Debug)]
pub struct Advertisement {
    pub discovery: DiscoveryRequest,
    pub created_at: SystemTime,
    pub expires_in: u64,
}

#[derive(Debug)]
pub struct DiscoveryQueue {
    pub queue: RwLock<HashMap<String, Advertisement>>,
}
