use bhw_macro_types::{
    BadRequestResponder, DefaultWithError, ErrorBadRequestResponder, FromDieselError,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, BadRequestResponder, DefaultWithError,
)]
pub struct SignInResponse {
    pub error: Option<SignInResponseError>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Error,
    ErrorBadRequestResponder,
    FromDieselError,
)]
pub enum SignInResponseError {
    #[error("Database error")]
    DBError,
    #[error("Username or password was incorrect")]
    UsernameOrPasswordIncorrect,
    #[error("Session error")]
    SessionError,
}
