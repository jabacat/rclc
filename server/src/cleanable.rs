use std::time::SystemTime;

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
        let mut to_remove: Vec<String> = vec![];
        match self.queue.try_read().map_err(|_| CleanError::LockError) {
            Ok(it) => {
                for ele in it.iter() {
                    match SystemTime::now().duration_since(ele.1.created_at + ele.1.expires_in) {
                        Ok(_) => {
                            // Ok signifies that the duration since the specified time was
                            // positive, therefor the advertisement is expired
                            to_remove.push(ele.0.to_string());
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
        Ok(to_remove
            .len()
            .try_into()
            .expect("Length of removed items was not expected to be so long")) // This can only
                                                                               // fail if the
                                                                               // unsigned value of
                                                                               // the number of
                                                                               // advertisements
                                                                               // removed exceeds
                                                                               // the 16 bit
                                                                               // integer limit
    }
}
