use serde::{Deserialize, Serialize};
use thiserror::Error;

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
