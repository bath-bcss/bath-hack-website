use bhw_types::{
    nothing::Nothing,
    requests::{
        profile::{ProfileResponse, ProfileResponseError},
        update_profile::UpdateProfileRequest,
    },
};

use super::api::{send_get, send_post, FrontendRequestError};

pub async fn get_profile() -> Result<ProfileResponse, FrontendRequestError<ProfileResponseError>> {
    send_get("/profile".to_string()).await
}

pub async fn update_profile(
    request: &UpdateProfileRequest,
) -> Result<Nothing, FrontendRequestError<ProfileResponseError>> {
    send_post("/profile".to_string(), request).await
}
