use std::error::Error;

use bhw_types::requests::sign_up::{SignUpRequest, SignUpResponse};
use gloo_net::http::Request;
use serde::Deserialize;

use super::api::build_path;

#[derive(Debug, Deserialize)]
pub enum SignUpError {
    SerializeFailed(String),
    RequestFailed(String),
    DeserializeFailed(String),
}

impl Error for SignUpError {}
impl std::fmt::Display for SignUpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SignUpError::SerializeFailed(err) => write!(f, "serializing request failed: {err}"),
            SignUpError::RequestFailed(err) => write!(f, "request failed: {err}"),
            SignUpError::DeserializeFailed(err) => write!(f, "deserialize failed: {err}"),
        }
    }
}

pub async fn sign_up_request(request: SignUpRequest) -> Result<SignUpResponse, SignUpError> {
    let resp: SignUpResponse = Request::post(build_path("/auth/signup".to_string()).as_str())
        .json(&request)
        .map_err(|e| SignUpError::SerializeFailed(e.to_string()))?
        .send()
        .await
        .map_err(|e| SignUpError::RequestFailed(e.to_string()))?
        .json()
        .await
        .map_err(|e| SignUpError::DeserializeFailed(e.to_string()))?;

    Ok(resp)
}
