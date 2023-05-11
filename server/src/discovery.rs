use super::structures::DiscoveryRequest;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, SystemTime};

/// Represents an Advertisement that is saved for a certain period of time.
#[derive(Debug)]
pub struct Advertisement {
    /// The request associated with this Advertisement
    pub discovery: DiscoveryRequest,
    /// The time the Advertisement was created
    pub created_at: SystemTime,
    /// How long until the Advertisement expires
    pub expires_in: Duration,
}

/// Manages the state of the application for persisting Advertisements
#[derive(Debug)]
pub struct DiscoveryQueue {
    pub queue: RwLock<HashMap<String, Advertisement>>,
}
