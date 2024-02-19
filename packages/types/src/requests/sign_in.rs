use bhw_macro_types::{FromDieselError, ResponseError, FromBlockingError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Error, FromDieselError, FromBlockingError, ResponseError)]
pub enum SignInResponseError {
    #[error("Database error")]
    DBError,
    #[error("Username or password was incorrect")]
    UsernameOrPasswordIncorrect,
    #[error("Session error")]
    SessionError,
}

pub type SignInResult = Result<Nothing, SignInResponseError>;
