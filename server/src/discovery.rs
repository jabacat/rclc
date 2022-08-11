use super::structures::DiscoveryRequest;
use rocket::serde::Serialize;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::SystemTime;

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
