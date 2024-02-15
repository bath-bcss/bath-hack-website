use figment::{providers::Env, Figment};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub bind_address: String,
    pub bind_port: u16,
    pub database_url: String,

    pub mailgun_api_key: String,
    pub mailgun_domain: String,
}

pub fn parse_config() -> AppConfig {
    Figment::new()
        .merge(Env::prefixed("BHW_").global())
        .extract()
        .unwrap()
}
