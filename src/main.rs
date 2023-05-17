// Macro's are used to define routes and the main function
use ::rocket::serde::json::{json, Value};
use auth::BasicAuth;
use rocket::{self, catch, catchers, delete, get, post, put, response::status, routes}; // import of rocket crate and macro's
mod auth; // import of auth.rs

#[catch(404)] // catching a 404 error
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found."})
}

#[get("/user")] // macro to define a route for all the resources
fn get_user(_auth: BasicAuth) -> Value {
    json!([{"id": 1, "name": "Winston Muijs"}, {"id": 2, "name": "Marcia Muijs"}])
}

#[get("/user/<id>")] // macro to define a route to a userwih id
fn view_user(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Winston Muijs", "email" : "whmuijs@gmail.com"})
}

#[post("/user", format = "json")] // macro to define a route to create a user
fn create_user(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "Michiel  Muijs", "email" : "wmuijs@xs4all.nl"})
}

#[put("/user/<id>", format = "json")] // macro to define a route for updating a user with id
fn update_user(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Winston Muijs", "email" : "john@doe.com"})
}

#[delete("/user/<_id>")] // macro to define a route for deleting a user with id
fn delete_user(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}
// macro to define a route
#[rocket::main] // macro to define the main function
async fn main() {
    // async main function
    let _ = rocket::build()
        .mount(
            "/",
            routes![get_user, view_user, create_user, update_user, delete_user],
        )
        .register("/", catchers![not_found])
        .launch()
        .await; // start the rocket server
}
