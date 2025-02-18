use bhw_types::{
    nothing::Nothing,
    requests::{
        activate::{AccountActivateRequest, AccountActivateResponseError},
        sign_up::{PossibleSignUpResponse, SignUpRequest, SignUpResponseError},
    },
};

use super::api::{send_post, FrontendRequestError};

pub async fn sign_up_request(
    request: &SignUpRequest,
) -> Result<PossibleSignUpResponse, FrontendRequestError<SignUpResponseError>> {
    send_post("/auth/signup".to_string(), &request).await
}

pub async fn account_activate_request(
    request: &AccountActivateRequest,
) -> Result<Nothing, FrontendRequestError<AccountActivateResponseError>> {
    send_post("/auth/activate".to_string(), request).await
}
