use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
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
