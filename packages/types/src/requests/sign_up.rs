use bhw_macro_types::{
    BadRequestResponder, DefaultWithError, ErrorBadRequestResponder, FromDieselError,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignUpRequest {
    pub bath_username: String,
}

#[derive(
    Debug, Clone, PartialEq, Deserialize, Serialize, BadRequestResponder, DefaultWithError,
)]
pub struct SignUpResponse {
    pub error: Option<SignUpResponseError>,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Deserialize,
    Serialize,
    Error,
    ErrorBadRequestResponder,
    FromDieselError,
)]
pub enum SignUpResponseError {
    #[error("Username already registered")]
    UsernameAlreadyExists,
    #[error("{0}")]
    CreateError(String),
    #[error("Cannot send email: {0}")]
    EmailError(String),
    #[error("Database failure")]
    DBError,
}
