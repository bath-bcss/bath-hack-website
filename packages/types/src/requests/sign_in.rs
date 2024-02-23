use bhw_macro_types::{FromSeaORMError, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SignInRequest {
    pub username: String,
    pub password: String,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Error, ResponseError, FromSeaORMError,
)]
pub enum SignInResponseError {
    #[error("Database error")]
    DBError,
    #[error("Username or password was incorrect")]
    UsernameOrPasswordIncorrect,
    #[error("Session error")]
    SessionError,
    #[error("User not a student")]
    UserNotStudentError,
    #[error("User not found in LDAP database")]
    PhantomUserError,
}

pub type SignInResult = Result<Nothing, SignInResponseError>;
