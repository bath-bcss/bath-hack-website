use bhw_macro_types::{JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, JsonResponder)]
pub struct GetCVDownloadURLResponse {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Error, ResponseError)]
pub enum GetCVDownloadURLResponseError {
    #[error("File handling error")]
    FileError,
}

pub type GetCVDownloadURLResult = Result<GetCVDownloadURLResponse, GetCVDownloadURLResponseError>;
