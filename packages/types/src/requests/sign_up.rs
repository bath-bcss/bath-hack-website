use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignUpRequest {
    pub bath_username: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignUpResponse {
    pub error: Option<SignUpError>,
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Error)]
pub enum SignUpError {
    #[error("Username already registered")]
    UsernameAlreadyExists,
    #[error("{0}")]
    CreateError(String),
    #[error("Database failure")]
    DBError,
}

#[cfg(target_family = "unix")]
impl actix_web::Responder for SignUpResponse {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let mut response = match self.error {
            Some(_) => actix_web::HttpResponse::BadRequest(),
            None => actix_web::HttpResponse::Ok(),
        };

        response
            .content_type(actix_web::http::header::ContentType::json())
            .json(self)
    }
}

#[cfg(target_family = "unix")]
impl actix_web::ResponseError for SignUpError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let s = SignUpResponse { error: Some(self.clone()) };
        actix_web::HttpResponse::BadRequest()
            .content_type(actix_web::http::header::ContentType::json())
            .json(s)
    }
}

impl Default for SignUpResponse {
    fn default() -> Self {
        SignUpResponse { error: None }
    }
}

#[cfg(target_family = "unix")]
impl From<diesel::result::Error> for SignUpError {
    fn from(_: diesel::result::Error) -> Self {
        SignUpError::DBError
    }
}
