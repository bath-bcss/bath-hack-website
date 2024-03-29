use bhw_macro_types::{FromSeaORMError, JsonResponder, ResponseError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::website_user::TShirtSize;

#[derive(Debug, Serialize, Deserialize, JsonResponder, Clone, PartialEq)]
pub struct ProfileResponse {
    pub bath_username: String,
    pub display_name: Option<String>,
    pub accessibility_requirements: Option<String>,
    pub dietary_requirements: Option<String>,
    pub t_shirt_size: Option<TShirtSize>,
}

#[derive(Debug, Serialize, Deserialize, Error, FromSeaORMError, ResponseError)]
pub enum ProfileResponseError {
    #[error("Database error")]
    DBError,
    #[error("Could not parse ID: {0}")]
    InvalidID(String),
    #[error("User not found")]
    NotFound,
}

pub type ProfileResult = Result<ProfileResponse, ProfileResponseError>;
