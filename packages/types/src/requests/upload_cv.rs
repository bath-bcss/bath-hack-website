use bhw_macro_types::ResponseError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::nothing::Nothing;

#[cfg(target_family = "unix")]
#[cfg_attr(target_family = "unix", derive(actix_multipart::form::MultipartForm))]
pub struct UploadCVRequest {
    pub cv_pdf: actix_multipart::form::tempfile::TempFile,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Error, ResponseError)]
pub enum UploadCVResponseError {
    #[error("File handling error")]
    FileError,
    #[error("File must be a PDF and no more than 10MB")]
    FileTypeOrSize,
}

pub type UploadCVResult = Result<Nothing, UploadCVResponseError>;
