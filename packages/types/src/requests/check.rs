use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckAuthResponse {
    pub signed_in: bool
}
