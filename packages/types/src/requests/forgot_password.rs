use bhw_macro_types::{FromSeaORMError, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForgotPasswordRequest {
    pub bath_username: String,
}

#[derive(
    Clone, Debug, PartialEq, Serialize, Deserialize, ResponseError, Error, FromSeaORMError,
)]
pub enum ForgotPasswordResponseError {
    #[error("Database error")]
    DBError,
    #[error("Blocking error")]
    BlockingError,
}

pub type ForgotPasswordResult = Result<Nothing, ForgotPasswordResponseError>;
