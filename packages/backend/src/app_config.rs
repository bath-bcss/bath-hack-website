use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub bind_address: String,
    pub bind_port: u16,
    pub database_url: String,

    pub redis_string: String,
    pub cookie_secret: String,

    pub mailgun_api_key: String,
    pub mailgun_domain: String,

    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key: String,
    pub s3_secret_key: String,

    #[cfg(feature = "ldap")]
    pub ldap_url: String,

    pub disable_signup: bool,
    pub use_unverified_usernames: bool,

    pub allowed_origin: String,
    pub dev_weak_passwords: bool,
    pub dev_skip_emails: bool,
}

pub fn parse_config() -> AppConfig {
    Figment::new()
        .merge(Env::prefixed("BHW_").global())
        .extract()
        .unwrap()
}
