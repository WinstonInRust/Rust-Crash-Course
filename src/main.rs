// Macro's are used to define routes and the main function
use ::rocket::serde::json::{json, Value};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{self, catch, catchers, delete, get, post, put, response::status, routes}; // import of rocket crate and macro's
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum BasicAuthError {
    InvalidHeader,
    InvalidBase64,
    InvalidFormat,
    HashingError(bcrypt::BcryptError),
}

impl Display for BasicAuthError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            BasicAuthError::InvalidHeader => write!(f, "Invalid authorization header"),
            BasicAuthError::InvalidBase64 => write!(f, "Invalid base64 encoding"),
            BasicAuthError::InvalidFormat => write!(f, "Invalid format"),
            BasicAuthError::HashingError(e) => write!(f, "Password hashing error: {}", e),
        }
    }
}

impl Error for BasicAuthError {}
pub struct BasicAuth {
    pub username: String,
    pub password_hash: String,
}

impl BasicAuth {
    fn from_authorization_header(header: &str) -> Result<BasicAuth, BasicAuthError> {
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(BasicAuthError::InvalidHeader);
        }
        if split[0] != "Basic" {
            return Err(BasicAuthError::InvalidHeader);
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Result<BasicAuth, BasicAuthError> {
        let decoded = base64::decode(base64_string).map_err(|_| BasicAuthError::InvalidBase64)?;
        let decoded = String::from_utf8(decoded).map_err(|_| BasicAuthError::InvalidBase64)?;
        let split = decoded.split(":").collect::<Vec<_>>();
        if split.len() != 2 {
            return Err(BasicAuthError::InvalidFormat);
        }
        let password_hash = hash_password(split[1])?;
        Ok(BasicAuth {
            username: split[0].to_string(),
            password_hash,
        })
    }
}

fn hash_password(password: &str) -> Result<String, BasicAuthError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(BasicAuthError::HashingError)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = BasicAuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        match auth_header {
            Some(auth_header) => match Self::from_authorization_header(auth_header) {
                Ok(auth) => Outcome::Success(auth),
                Err(e) => Outcome::Failure((Status::BadRequest, e)),
            },
            None => Outcome::Failure((Status::Unauthorized, BasicAuthError::InvalidHeader)),
        }
    }
}

#[catch(404)] // catching a 404 error
fn not_found() -> Value {
    json!({"status": "error", "reason": "Resource was not found."})
}

#[get("/user")] // macro to define a route for all the resources
fn get_user(_auth: BasicAuth) -> Value {
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
