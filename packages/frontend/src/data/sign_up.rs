use bhw_types::requests::{sign_up::{SignUpRequest, SignUpResponse}, activate::{AccountActivateRequest, AccountActivateResponse}};

use super::api::{FrontendRequestError, send_post};

pub async fn sign_up_request(request: &SignUpRequest) -> Result<SignUpResponse, FrontendRequestError> {
    send_post("/auth/signup".to_string(), &request).await
}

pub async fn account_activate_request(request: &AccountActivateRequest) -> Result<AccountActivateResponse, FrontendRequestError> {
    send_post("/auth/activate".to_string(), request).await
}
