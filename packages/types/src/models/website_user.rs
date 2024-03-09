use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, EnumIter)]
pub enum TShirtSize {
    S,
    M,
    L,
    XL,
    XXL,
    XXXL,
    XXXXL,
}
