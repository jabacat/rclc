use notify_rust::Notification;

fn main() {

    println!("Hello, world!");

    Notification::new()
    .summary("RCLC")
    .body("The RCLC server has been launched!")
    .show();

}
