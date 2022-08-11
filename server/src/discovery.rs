use rocket::serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::sync::RwLock;
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(FromForm, Debug, Deserialize, Serialize, Clone)]
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

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum Status {
    Match,
    NoMatch,
    Failure,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DiscoveryResponse {
    pub status: Status,
    pub error: Option<String>,
    pub discovery: Option<DiscoveryRequest>,
    pub message: String,
}
