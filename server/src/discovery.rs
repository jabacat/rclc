use super::structures::DiscoveryRequest;
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

