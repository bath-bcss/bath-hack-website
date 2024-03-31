use bhw_macro_types::{JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonResponder)]
pub struct CVExistsResponse {
    pub exists: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, ResponseError, Error)]
pub enum CVExistsResponseError {
    #[error("File handling error")]
    FileError,
}

pub type CVExistsResult = Result<CVExistsResponse, CVExistsResponseError>;
