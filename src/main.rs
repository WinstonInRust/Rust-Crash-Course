// Macro's are used to define routes and the main function
use ::rocket::serde::json::{json, Value};
use rocket::{self, get, routes}; // import of rocket crate and macro's // import of json macro // import of json value

#[get("/")] // macro to define a route
fn welkom() -> Value {
    json!("Welkom op mijn site!!")
}

#[rocket::main] // macro to define the main function
async fn main() {
    // async main function
    let _ = rocket::build().mount("/", routes![welkom]).launch().await; // start the rocket server
}
