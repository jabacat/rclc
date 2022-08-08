use notify_rust::Notification;

fn main() {

    println!("Hello, world!");

    Notification::new()
    .summary("RCLC")
    .body("The RCLC daemon has been launched!")
    .show();

}
