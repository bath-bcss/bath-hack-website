use bhw_macro_types::{FromSeaORMError, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_family = "unix")]
use validator::Validate;

use crate::nothing::Nothing;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(target_family = "unix", derive(Validate))]
pub struct RenameGroupRequest {
    #[cfg_attr(
        target_family = "unix",
        validate(
            length(min = 2, max = 20, message = "Must be between 2 and 20 characters"),
            regex(
                path = "crate::validation::RE_ALPHANUM",
                message = "Must be alphanumeric"
            )
        )
    )]
    pub new_name: String,
}

#[derive(
    Debug, Clone, PartialEq, Serialize, Deserialize, Error, ResponseError, FromSeaORMError,
)]
pub enum RenameGroupResponseError {
    #[error("Database error")]
    DBError,
    #[error("User not found")]
    UserNotFound,
    #[error("User not in group")]
    UserNotInGroup,
}

pub type RenameGroupResult = Result<Nothing, RenameGroupResponseError>;
