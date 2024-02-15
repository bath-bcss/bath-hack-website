use serde::Deserialize;
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
