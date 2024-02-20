use bhw_macro_types::{JsonResponder, ResponseError, FromDieselError, FromBlockingError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonResponder)]
pub enum UpdateProfileRequest {
    DisplayName(Option<String>),
    AccessibilityRequirements(Option<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Error, ResponseError, FromDieselError, FromBlockingError)]
pub enum UpdateProfileResponseError {
    #[error("Database error")]
    DBError
}

pub type UpdateProfileResult = Result<Nothing, UpdateProfileResponseError>;
