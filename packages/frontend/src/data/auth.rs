use bhw_types::{
    nothing::Nothing,
    requests::{
        check::CheckAuthResponse,
        sign_in::{SignInRequest, SignInResponseError},
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
            match check_signed_in().await {
                Err(_) => $navigator.push(&Route::Login),
                Ok(resp) => match resp.signed_in {
                    true => return,
                    false => $navigator.push(&Route::Login),
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
