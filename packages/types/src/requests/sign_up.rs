use bhw_macro_types::{FromDieselError, ResponseError, FromBlockingError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SignUpRequest {
    pub bath_username: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Error, FromDieselError, FromBlockingError, ResponseError)]
pub enum SignUpResponseError {
    #[error("Username already registered")]
    UsernameAlreadyExists,
    #[error("That doesn't look like a valid UoB username. Please contact su-bcss@bath.ac.uk if we're wrong.")]
    UsernameInvalid,
    #[error("{0}")]
    CreateError(String),
    #[error("Cannot send email: {0}")]
    EmailError(String),
    #[error("Database failure")]
    DBError,
}

pub type SignUpResult = Result<Nothing, SignUpResponseError>;
