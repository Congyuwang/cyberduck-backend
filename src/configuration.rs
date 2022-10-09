use crate::redis_session_layer::RedisSessionConfig;
use config::{Config, ConfigError};
use serde::Deserialize;

pub const CONFIG_FILE_NAME: &str = "config.yaml";

#[derive(Deserialize)]
pub struct Configuration {
    pub redis_session: RedisSessionConfig,
    pub server_binding: String,
    pub admin_token: String,
    pub server_tls: Option<TlsConfig>,
    pub db_url: String,
}

#[derive(Deserialize)]
pub struct TlsConfig {
    pub key: String,
    pub cert: String,
}

impl Configuration {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(config::File::with_name(CONFIG_FILE_NAME))
            .build()?;
        config.try_deserialize()
    }
}
