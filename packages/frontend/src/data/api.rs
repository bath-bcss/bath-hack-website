use bhw_types::{auth::AuthSessionError, validation::ValidationError};
use gloo_console::warn;
use gloo_net::http::{Request, Response};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;
use web_sys::{Blob, File, FormData, RequestCredentials};

#[derive(Debug, Deserialize, Error)]
pub enum FrontendRequestError<E> {
    #[error("serializing failed: {0}")]
    SerializeFailed(String),
    #[error("network error: {0}")]
    RequestFailed(String),
    #[error("deserialize failed: {0}")]
    DeserializeFailed(String),
    #[error("{0}")]
    ValidationFailed(String),
    #[error("Auth: {0}")]
    AuthenticationFailed(#[from] AuthSessionError),
    #[error("{0}")]
    GenericError(E),
}

pub fn build_path(path: String) -> String {
    let base_url = std::env!("BHW_FRONTEND_API_URL").to_string();
    base_url + path.as_str()
}

async fn handle_response<Res, Error>(response: Response) -> Result<Res, FrontendRequestError<Error>>
where
    Res: DeserializeOwned,
    Error: DeserializeOwned,
{
    if response.status() >= 200 && response.status() < 300 {
        let data: Res = response
            .json()
            .await
            .map_err(|e| FrontendRequestError::DeserializeFailed(e.to_string()))?;
        Ok(data)
    } else if response.status() == 401 {
        let data: AuthSessionError = response
            .json()
            .await
            .map_err(|e| FrontendRequestError::DeserializeFailed(e.to_string()))?;
        return Err(data.into());
    } else if response.status() == 400 {
        let data: ValidationError = response
            .json()
            .await
            .map_err(|e| FrontendRequestError::DeserializeFailed(e.to_string()))?;
        Err(FrontendRequestError::ValidationFailed(data.to_string()))
    } else {
        let data: Error = response
            .json()
            .await
            .map_err(|e| FrontendRequestError::DeserializeFailed(e.to_string()))?;
        Err(FrontendRequestError::GenericError(data))
    }
}
pub async fn send_get<Res, Error>(path: String) -> Result<Res, FrontendRequestError<Error>>
where
    Res: DeserializeOwned,
    Error: DeserializeOwned,
{
    let resp = Request::get(build_path(path).as_str())
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| FrontendRequestError::RequestFailed(e.to_string()))?;

    handle_response(resp).await
}

pub async fn send_post<Req, Res, Error>(
    path: String,
    data: &Req,
) -> Result<Res, FrontendRequestError<Error>>
where
    Req: Serialize,
    Res: DeserializeOwned,
    Error: DeserializeOwned,
{
    let resp = Request::post(build_path(path).as_str())
        .credentials(RequestCredentials::Include)
        .json(&data)
        .map_err(|e| FrontendRequestError::SerializeFailed(e.to_string()))?
        .send()
        .await
        .map_err(|e| FrontendRequestError::RequestFailed(e.to_string()))?;

    handle_response(resp).await
}

pub async fn send_file<Res, Error>(
    path: String,
    file: File,
    key: &str,
) -> Result<Res, FrontendRequestError<Error>>
where
    Res: DeserializeOwned,
    Error: DeserializeOwned,
{
    let blob: Blob = file.into();
    let fd = FormData::new().expect("Create new FormData");
    fd.set_with_blob(key, &blob).map_err(|e| {
        warn!(e);
        FrontendRequestError::SerializeFailed("set_with_blob (see console)".to_string())
    })?;

    let resp = Request::post(build_path(path).as_str())
        .credentials(RequestCredentials::Include)
        .body(fd)
        .map_err(|e| FrontendRequestError::SerializeFailed(e.to_string()))?
        .send()
        .await
        .map_err(|e| FrontendRequestError::RequestFailed(e.to_string()))?;

    handle_response(resp).await
}
