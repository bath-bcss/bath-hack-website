use bhw_macro_types::ResponseError;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Error, ResponseError)]
pub enum SignOutResponseError {
}

pub type SignOutResult = Result<Nothing, SignOutResponseError>;
