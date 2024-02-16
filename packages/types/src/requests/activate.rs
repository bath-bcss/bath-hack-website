use bhw_macro_types::{BadRequestResponder, DefaultWithError, ErrorBadRequestResponder, FromDieselError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountActivateRequest {
    pub id: String,
    pub secret: String,
    pub password: String,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, BadRequestResponder, DefaultWithError,
)]
pub struct AccountActivateResponse {
    error: Option<AccountActivateResponseError>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Error, ErrorBadRequestResponder, FromDieselError)]
pub enum AccountActivateResponseError {
    #[error("ID or secret was wrong")]
    IdOrSecretWrong,
    #[error("Secret checking failed")]
    SecretError,
    #[error("Database error")]
    DBError,
    #[error("Creating user: {0}")]
    CreateUserError(String),
    #[error("SessionError")]
    SessionError,
}
