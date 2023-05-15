// Macro's are used to define routes and the main function
use ::rocket::serde::json::{json, Value};
use rocket::{self, delete, get, post, put, response::status, routes}; // import of rocket crate and macro's // import of json macro // import of json value

#[get("/user")] // macro to define a route for all the resources
fn get_user() -> Value {
    json!([{"id": 1, "name": "Winston Muijs"}, {"id": 2, "name": "Marcia Muijs"}])
}

#[get("/user/<id>")] // macro to define a route to a userwih id
fn view_user(id: i32) -> Value {
    json!({"id": id, "name": "Winston Muijs", "email" : "whmuijs@gmail.com"})
}

#[post("/user", format = "json")] // macro to define a route to create a user
fn create_user() -> Value {
    json!({"id": 3, "name": "Michiel  Muijs", "email" : "wmuijs@xs4all.nl"})
}

#[put("/user/<id>", format = "json")] // macro to define a route for updating a user with id
fn update_user(id: i32) -> Value {
    json!({"id": id, "name": "Winston Muijs", "email" : "john@doe.com"})
}

#[delete("/user/<id>")] // macro to define a route for deleting a user with id
fn delete_user(id: i32) -> status::NoContent {
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
        .launch()
        .await; // start the rocket server
}
