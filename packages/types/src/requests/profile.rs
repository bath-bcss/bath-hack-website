use bhw_macro_types::{FromDieselError, ResponseError};
use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileResponse {
    bath_username: String,
    display_name: String,
}

#[derive(Debug, Serialize, Deserialize, Error, FromDieselError, ResponseError)]
pub enum ProfileResponseError {
    #[error("Database error")]
    DBError,
}

pub type ProfileResult = Result<ProfileResponse, ProfileResponseError>;
