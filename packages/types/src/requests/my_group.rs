use bhw_macro_types::{FromSeaORMError, JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::group::GroupMember;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MyGroupData {
    pub id: String,
    pub name: String,
    pub join_code: String,
    pub members: Vec<GroupMember>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonResponder)]
pub enum MyGroupResponse {
    None,
    Data(MyGroupData),
}

#[derive(
    Debug, Clone, PartialEq, Deserialize, Serialize, Error, ResponseError, FromSeaORMError,
)]
pub enum MyGroupResponseError {
    #[error("Database error")]
    DBError,
    #[error("Your user account does not exist (somewhat strangely)")]
    UserNotFound,
}

pub type MyGroupResult = Result<MyGroupResponse, MyGroupResponseError>;
