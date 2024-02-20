use bhw_macro_types::{FromDieselError, ResponseError, JsonResponder, FromBlockingError};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, JsonResponder, Clone, PartialEq)]
pub struct ProfileResponse {
    pub bath_username: String,
    pub display_name: Option<String>,
    pub accessibility_requirements: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Error, FromDieselError, FromBlockingError, ResponseError)]
pub enum ProfileResponseError {
    #[error("Database error")]
    DBError,
    #[error("Could not parse ID: {0}")]
    InvalidID(String),
    #[error("User not found")]
    NotFound,
}

pub type ProfileResult = Result<ProfileResponse, ProfileResponseError>;
