use std::time::{Duration, SystemTime};

use crate::discovery::DiscoveryQueue;

pub enum CleanError {
    LockError,
}

pub trait Cleanable {
    // Returns the amount of instances that have been cleaned from the queue
    fn clean(&self) -> Result<u16, CleanError>;
}

impl Cleanable for DiscoveryQueue {
    fn clean(&self) -> Result<u16, CleanError> {
        match self.queue.try_read().map_err(|e| CleanError::LockError) {
            Ok(it) => {
                for ele in it.values() {
                    SystemTime::now().checked_add(ele.expires_in);
                }
            }
            Err(err) => return Err(err),
        };
        Ok(0)
    }
}
