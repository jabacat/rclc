use notify_rust::Notification;

/**
 * This is just a thin wrapper around the Notification stuff for now.
 * The library says it always returns Ok, so we're not worrying about anything
 * else at the moment.
 */
pub fn notif(summary: &str, msg: &str) {
    match Notification::new().summary(summary).body(msg).show() {
        Ok(_) => (),
        // This should never happen. TODO: log this event which supposedly
        // never happens.
        Err(_) => eprintln!("Unable to display notification!"),
    }
}
