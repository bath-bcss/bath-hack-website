use bhw_types::{
    nothing::Nothing,
    requests::{
        check::CheckAuthResponse,
        forgot_password::{ForgotPasswordRequest, ForgotPasswordResponseError},
        forgot_password_pin::{ForgotPasswordPINRequest, ForgotPasswordPINResponseError},
        sign_in::{SignInRequest, SignInResponseError},
        sign_out::SignOutResponseError,
    },
};

use super::api::{send_get, send_post, FrontendRequestError};

pub async fn check_signed_in() -> Result<CheckAuthResponse, FrontendRequestError<()>> {
    send_get("/auth/check".to_string()).await
}

#[macro_export]
macro_rules! redirect_if_not_authed {
    ($navigator: ident) => {
        wasm_bindgen_futures::spawn_local(async move {
            match $crate::data::auth::check_signed_in().await {
                Err(_) => $navigator.push(&$crate::router::Route::Login),
                Ok(resp) => match resp.signed_in {
                    true => return,
                    false => $navigator.push(&$crate::router::Route::Login),
                },
            }
        });
    };
}

pub async fn sign_in(
    username: String,
    password: String,
) -> Result<Nothing, FrontendRequestError<SignInResponseError>> {
    let request = SignInRequest { username, password };
    send_post("/auth/signin".to_string(), &request).await
}

pub async fn sign_out() -> Result<Nothing, FrontendRequestError<SignOutResponseError>> {
    send_post("/auth/signout".to_string(), &Nothing).await
}

pub async fn forgot_password(
    username: String,
) -> Result<Nothing, FrontendRequestError<ForgotPasswordResponseError>> {
    let request = ForgotPasswordRequest {
        bath_username: username,
    };
    send_post("/auth/reset/password".to_string(), &request).await
}

pub async fn forgot_password_pin(
    pin: String,
    new_password: String,
) -> Result<Nothing, FrontendRequestError<ForgotPasswordPINResponseError>> {
    let request = ForgotPasswordPINRequest { pin, new_password };
    send_post("/auth/reset/password/pin".to_string(), &request).await
}
