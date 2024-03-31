use bhw_macro_types::ResponseError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Error, ResponseError)]
pub enum DeleteCVResponseError {
    #[error("File handling error")]
    FileError,
}

pub type DeleteCVResult = Result<Nothing, DeleteCVResponseError>;
