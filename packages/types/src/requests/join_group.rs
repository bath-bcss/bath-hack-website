use bhw_macro_types::{FromSeaORMError, JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::group::GroupMember;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct JoinGroupRequest {
    pub join_code: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonResponder)]
pub struct JoinGroupResponse {
    pub group_id: String,
    pub group_name: String,
    pub group_number: u64,
    pub group_members: Vec<GroupMember>,
}

#[derive(
    Debug, Clone, PartialEq, Deserialize, Serialize, Error, ResponseError, FromSeaORMError,
)]
pub enum JoinGroupError {
    #[error("You're already in a group. Please leave it first.")]
    AlreadyInGroup,
    #[error("That join code was not found")]
    CodeNotFound,
    #[error("That group has already reached the maximum number of members")]
    MaxCapacity,
    #[error("Database error")]
    DBError,
}

pub type JoinGroupResult = Result<JoinGroupResponse, JoinGroupError>;
