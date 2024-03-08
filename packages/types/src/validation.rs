use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_family = "unix")]
pub static RE_ALPHANUM: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^([a-z]|[A-Z]|[0-9]| )*$").unwrap());

#[cfg(target_family = "unix")]
pub static RE_NAME: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"^([a-z]|[A-Z]|[0-9]| |'|-|_)*$").unwrap());

#[derive(Debug, Clone, PartialEq, Error, Serialize, Deserialize)]
pub enum ValidationError {
    #[error("Parsing JSON payload: {0}")]
    Parse(String),
    #[error("{0}")]
    Values(String),
    #[error("Unkown validation error")]
    Unknown,
}

#[cfg(target_family = "unix")]
impl actix_web::ResponseError for ValidationError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponse::BadRequest().json(self)
    }
}
