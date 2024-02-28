use bhw_macro_types::{FromSeaORMError, ResponseError, JsonResponder};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateGroupRequest {
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
