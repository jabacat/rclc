use common::notif::notif;
use efcl::{bold, color, Color};
use std::io::{stdin, stdout, Write};

pub mod hook;

pub fn interactive() {
    let mut input;

    println!("{}", bold!("Rust Command Line Chat (RCLC)"));
    println!("{}", color!(Color::BLUE, "Interactive Mode"));

    loop {
        print!("{}", color!(Color::GREEN, "\n> "));
        stdout().flush().expect("Failed to flush stdout");

        input = String::new();

        hook::hook("/tmp/rclc.sock");

        match stdin().read_line(&mut input) {
            Ok(_) => {
                input.pop();
                notif("Message", input.as_str());
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

fn main() {
    interactive();
}
