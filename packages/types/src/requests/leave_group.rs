use bhw_macro_types::{FromSeaORMError, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

pub type LeaveGroupRequest = Nothing;

#[derive(
    Debug, Clone, PartialEq, Deserialize, Serialize, Error, ResponseError, FromSeaORMError,
)]
pub enum LeaveGroupResponseError {
    #[error("Database error")]
    DBError,
    #[error("You are not in a group")]
    NotInGroup,
    #[error("Account was not found")]
    UserNotFound
}

pub type LeaveGroupResult = Result<Nothing, LeaveGroupResponseError>;
