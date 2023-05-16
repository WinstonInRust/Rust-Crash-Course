// Macro's are used to define routes and the main function
use ::rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{self, catch, catchers, delete, get, post, put, response::status, routes}; // import of rocket crate and macro's // import of json macro // import of json value

pub struct BasicAuth {
    pub username: String,
    pub password: String,
}

impl BasicAuth {
    fn from_autherization_header(header: &str) -> Option<BasicAuth> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }
        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded = base64::decode(base64_string).ok()?;
        let decoded = String::from_utf8(decoded).ok()?;
        let split = decoded.split(":").collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }
        Some(BasicAuth {
            username: split[0].to_string(),
            password: split[1].to_string(),
        })
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header {
            if let Some(auth) = Self::from_autherization_header(auth_header) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}

#[catch(404)] // catching a 404 error
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found."})
}

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

#[delete("/user/<_id>")] // macro to define a route for deleting a user with id
fn delete_user(_id: i32) -> status::NoContent {
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
