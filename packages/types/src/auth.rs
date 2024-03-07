use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AuthSessionError {
    #[error("Reading session: {0}")]
    ReadingSession(String),
    #[error("Not signed in")]
    NotAuthenticated,
    #[error("Database not connected")]
    NotConnected,
    #[error("User ID was not valid because: {0}")]
    IDNotValid(String),
    #[error("Database error")]
    DBError(String),
}

#[cfg(target_family = "unix")]
impl actix_web::ResponseError for AuthSessionError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        actix_web::HttpResponse::Unauthorized()
            .content_type(actix_web::http::header::ContentType::json())
            .json(self)
    }
}
