use bhw_macro_types::{FromSeaORMError, JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JoinGroupRequest {
    pub join_code: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonResponder)]
pub struct JoinGroupResponse {
    pub group_id: String,
    pub group_name: String,
}

#[derive(
    Debug, Clone, PartialEq, Deserialize, Serialize, Error, ResponseError, FromSeaORMError,
)]
pub enum JoinGroupError {
    #[error("You're already in a group. Please leave it first.")]
    AlreadyInGroup,
    #[error("That join code was not found")]
    CodeNotFound,
    #[error("Database error")]
    DBError,
}

pub type JoinGroupResult = Result<JoinGroupResponse, JoinGroupError>;
