use bhw_macro_types::{FromSeaORMError, JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_family = "unix")]
use validator::Validate;

use crate::{models::website_user::TShirtSize, nothing::Nothing};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonResponder, Default)]
#[cfg_attr(target_family = "unix", derive(Validate))]
pub struct UpdateProfileRequest {
    #[cfg_attr(
        target_family = "unix",
        validate(
            length(max = 30, message = "Can't be longer than 30 characters"),
            regex(
                path = "crate::validation::RE_NAME",
                message = "Can only contain alphanumeric and ' - _"
            )
        )
    )]
    pub display_name: Option<String>,
    #[cfg_attr(
        target_family = "unix",
        validate(length(max = 1000, message = "Can't be longer than 1000 characters"),)
    )]
    pub accessibility_requirements: Option<String>,
    #[cfg_attr(
        target_family = "unix",
        validate(
            length(max = 1000, message = "Can't be longer than 1000 characters"),
            non_control_character
        )
    )]
    pub dietary_requirements: Option<String>,
    pub t_shirt_size: Option<TShirtSize>,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Error, ResponseError, FromSeaORMError,
)]
pub enum UpdateProfileResponseError {
    #[error("Database error")]
    DBError,
}

pub type UpdateProfileResult = Result<Nothing, UpdateProfileResponseError>;
