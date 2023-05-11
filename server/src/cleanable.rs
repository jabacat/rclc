use std::time::SystemTime;

use crate::discovery::DiscoveryQueue;

pub enum CleanError {
    LockError,
}

pub trait Cleanable {
    /// Cleans each advertisement based on it's age and expiration date
    /// Returns the number of advertisements removed
    fn clean(&self) -> Result<usize, CleanError>;
}

impl Cleanable for DiscoveryQueue {
    fn clean(&self) -> Result<usize, CleanError> {
        let mut to_remove: Vec<String> = vec![];
        match self.queue.try_read().map_err(|_| CleanError::LockError) {
            Ok(it) => {
                for ele in it.iter() {
                    match SystemTime::now().duration_since(ele.1.created_at + ele.1.expires_in) {
                        Ok(_) => {
                            // Ok signifies that the duration since the specified time was
                            // positive, therefor the advertisement is expired
                            to_remove.push(ele.0.to_string());
                            println!("Removing request by {}", ele.1.discovery.requested_by);
                        }
                        Err(_) => {
                            // The advertisement is not expired yet
                        }
                    }
                }
            }
            Err(err) => return Err(err),
        };

        for ele in &to_remove {
            self.queue
                .try_write()
                .map_err(|_| CleanError::LockError)?
                .remove(ele);
        }
        Ok(to_remove.len())
    }
}

#[cfg(test)]
mod test {
    use std::{
        collections::HashMap,
        net::{IpAddr, Ipv4Addr},
        sync::RwLock,
        time::{Duration, SystemTime},
    };

    use common::structures::DiscoveryRequest;

    use crate::discovery::{Advertisement, DiscoveryQueue};

    #[test]
    fn clean_test() {
        let discoveryqueue = DiscoveryQueue {
            queue: RwLock::new(HashMap::new()),
        };

        let mut locked_queue = discoveryqueue.queue.write().unwrap();

        let disc_req_a = DiscoveryRequest {
            ip: Some(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            port: 2121,
            requested_by: "A".to_string(),
            looking_for: "B".to_string(),
            public_key: "qwertyuiop".to_string(),
        };

        locked_queue.insert(
            "A".to_string(),
            Advertisement {
                discovery: disc_req_a,
                created_at: SystemTime::now(),
                expires_in: Duration::from_millis(100),
            },
        );
    }
}
