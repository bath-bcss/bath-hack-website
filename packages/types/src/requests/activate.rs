use bhw_macro_types::{FromBlockingError, FromDieselError, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccountActivateRequest {
    pub id: String,
    pub secret: String,
    pub password: String,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Deserialize,
    Serialize,
    Error,
    FromDieselError,
    FromBlockingError,
    ResponseError,
)]
pub enum AccountActivateResponseError {
    #[error("ID or secret was wrong")]
    IdOrSecretWrong,
    #[error("Your link has expired. Please go back to the signup page and start again.")]
    RequestExpired,
    #[error("{0}")]
    InsecurePassword(String),
    #[error("Secret checking failed")]
    SecretError,
    #[error("Database error")]
    DBError,
    #[error("Creating user: {0}")]
    CreateUserError(String),
    #[error("Deleting signup request: {0}")]
    DeleteRequestError(String),
    #[error("SessionError")]
    SessionError,
}

pub type AccountActivateResult = Result<Nothing, AccountActivateResponseError>;
