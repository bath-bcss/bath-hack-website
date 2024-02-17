use gloo_net::http::Request;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use thiserror::Error;

#[derive(Debug, Deserialize, Error)]
pub enum FrontendRequestError {
    #[error("serializing failed: `{0}`")]
    SerializeFailed(String),
    #[error("`{0}`")]
    RequestFailed(String),
    #[error("deserialize failed: `{0}`")]
    DeserializeFailed(String),
}

pub fn build_path(path: String) -> String {
    let base_url = std::env!("BHW_FRONTEND_API_URL").to_string();
    base_url + path.as_str()
}

pub async fn send_post<Req, Res>(path: String, data: &Req) -> Result<Res, FrontendRequestError>
where
    Req: Serialize,
    Res: DeserializeOwned
{
    let resp: Res = Request::post(build_path(path).as_str())
        .json(&data)
        .map_err(|e| FrontendRequestError::SerializeFailed(e.to_string()))?
        .send()
        .await
        .map_err(|e| FrontendRequestError::RequestFailed(e.to_string()))?
        .json()
        .await
        .map_err(|e| FrontendRequestError::DeserializeFailed(e.to_string()))?;

    Ok(resp)
}
