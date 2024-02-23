use bhw_macro_types::{FromSeaORMError, JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonResponder)]
pub enum UpdateProfileRequest {
    DisplayName(Option<String>),
    AccessibilityRequirements(Option<String>),
    DietaryRequirements(Option<String>),
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Error, ResponseError, FromSeaORMError,
)]
pub enum UpdateProfileResponseError {
    #[error("Database error")]
    DBError,
}

pub type UpdateProfileResult = Result<Nothing, UpdateProfileResponseError>;
