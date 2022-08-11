use reqwest;

use common::structures;

pub mod notif;
pub mod discovery;

fn main() {
    println!("Hello, world!");

    notif::notif("RCLC", "The RCLC daemon has been launched!");
}
