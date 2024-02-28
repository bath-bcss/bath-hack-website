use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct GroupMember {
    pub bath_username: String,
    pub display_name: Option<String>,
}
