use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct SignUpRequest {
    pub bath_username: String,
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct SignUpResponse {
    pub success: bool,
}
