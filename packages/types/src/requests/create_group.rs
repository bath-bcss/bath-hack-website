use bhw_macro_types::{FromSeaORMError, JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(target_family = "unix")]
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(target_family = "unix", derive(Validate))]
pub struct CreateGroupRequest {
    #[cfg_attr(
        target_family = "unix",
        validate(
            length(min = 2, max = 20, message = "Must be between 2 and 20 characters"),
            regex(
                path = *crate::validation::RE_ALPHANUM,
                message = "Must be alphanumeric"
            )
        )
    )]
    pub group_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonResponder)]
pub struct CreateGroupResponse {
    pub new_group_id: String,
    pub new_group_join_code: String,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, PartialEq, Error, ResponseError, FromSeaORMError,
)]
pub enum CreateGroupError {
    #[error("Database error")]
    DBError,
    #[error("You are already in a group. Please leave it to create a new one.")]
    AlreadyInGroup,
}

pub type CreateGroupResult = Result<CreateGroupResponse, CreateGroupError>;
