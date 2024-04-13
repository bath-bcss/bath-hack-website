use bhw_macro_types::{FromSeaORMError, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignUpRequest {
    pub bath_username: String,
}

#[derive(
    Debug, Clone, PartialEq, Deserialize, Serialize, Error, ResponseError, FromSeaORMError,
)]
pub enum SignUpResponseError {
    #[error("Username already registered")]
    UsernameAlreadyExists,
    #[error("That doesn't look like a valid UoB username. Please contact su-bcss@bath.ac.uk if we're wrong.")]
    UsernameInvalid,
    #[error("Sorry, it looks like you aren't a student at UoB. Please contact su-bcss@bath.ac.uk if we're wrong.")]
    UserIsNotStudent,
    #[error("{0}")]
    CreateError(String),
    #[error("Cannot send email: {0}")]
    EmailError(String),
    #[error("New signups are currently disabled. Please contact su-bcss@bath.ac.uk for support.")]
    SignupDisabled,
    #[error("Database failure")]
    DBError,
    #[error("Blocking error")]
    BlockingError,
}

pub type SignUpResult = Result<Nothing, SignUpResponseError>;
