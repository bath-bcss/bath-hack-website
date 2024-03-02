use bhw_macro_types::{FromSeaORMError, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ForgotPasswordPINRequest {
    pub pin: String,
    pub new_password: String,
}

#[derive(
    Clone, Debug, PartialEq, Deserialize, Serialize, Error, ResponseError, FromSeaORMError,
)]
pub enum ForgotPasswordPINResponseError {
    #[error("Database error")]
    DBError,
    #[error("PIN invalid or not found")]
    PIN,
    #[error("{0}")]
    InsecurePassword(String),
}

pub type ForgotPasswordPINResult = Result<Nothing, ForgotPasswordPINResponseError>;
