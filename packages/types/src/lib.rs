pub mod auth;
pub mod models;
pub mod nothing;
pub mod requests;
pub mod validation;

#[cfg(target_family = "unix")]
pub use actix_web_validator;
pub use strum;
#[cfg(target_family = "unix")]
pub use validator;
