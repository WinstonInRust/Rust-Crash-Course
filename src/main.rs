// Macro's are used to define routes and the main function
use rocket::{self, get, routes}; // import of rocket crate and macro's

#[get("/")] // macro to define a route
fn welkom() -> &'static str {
    "Welkom op mijn site!!\n"
}

#[rocket::main] // macro to define the main function
async fn main() {
    // async main function
    let _ = rocket::build().mount("/", routes![welkom]).launch().await; // start the rocket server
}
