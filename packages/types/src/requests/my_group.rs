use bhw_macro_types::{FromSeaORMError, JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonResponder)]
pub struct MyGroupResponse {
    pub id: String,
    pub name: String,
    pub join_code: String,
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

pub type MyGroupResult = Result<Option<MyGroupResponse>, MyGroupResponseError>;
