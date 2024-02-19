use bhw_macro_types::JsonResponder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, JsonResponder)]
pub struct Nothing;
