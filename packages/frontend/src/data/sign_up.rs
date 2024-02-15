use bhw_types::requests::sign_up::{SignUpRequest, SignUpResponse};
use gloo_net::http::Request;

use super::api::{build_path, FrontendRequestError};

pub async fn sign_up_request(request: SignUpRequest) -> Result<SignUpResponse, FrontendRequestError> {
    let resp: SignUpResponse = Request::post(build_path("/auth/signup".to_string()).as_str())
        .json(&request)
        .map_err(|e| FrontendRequestError::SerializeFailed(e.to_string()))?
        .send()
        .await
        .map_err(|e| FrontendRequestError::RequestFailed(e.to_string()))?
        .json()
        .await
        .map_err(|e| FrontendRequestError::DeserializeFailed(e.to_string()))?;

    Ok(resp)
}
