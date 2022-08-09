#[macro_use]
extern crate rocket;

use std::net::Ipv4Addr;

struct ChatRequest {
    ip: Ipv4Addr,
    userid: String,
}

#[get("/")]
fn home() -> String {
    "Hello Jabacat!".to_string()
}

// To setup a connection, a user will give their IP address and the username or userid of the
// person they want to talk to. This data will be put into the ChatRequest and then will be
// added to a queue or vector if the userid of the other person is not in the collection already
// if it is in the collection, then send the ip of the other person back as a response
#[get("/setup-connection")]
fn setup_connection() -> String {
    // 1. Get IP of the user and the (UID or username) of the user they want to talk to
    //
    // if userid is in the vector of ChatRequest
    //      1. Send the other persons IP address back as a response
    // else
    //      1. Create a ChatRequest { ip, userid }
    //      2. Add it to a queue or vector

    "Setup Connection".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home])
}
