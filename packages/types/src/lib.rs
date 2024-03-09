pub mod requests;
pub mod nothing;
pub mod validation;
pub mod models;
pub mod auth;

#[cfg(target_family = "unix")]
pub use actix_web_validator;
#[cfg(target_family = "unix")]
pub use validator;
pub use strum;
