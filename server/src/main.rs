#[macro_use]
extern crate rocket;

#[get("/")]
fn home() -> String {
    "Hello Jabacat!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![home])
}
