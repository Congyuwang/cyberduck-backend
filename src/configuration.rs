use crate::redis_session_layer::RedisSessionConfig;
use crate::wechat_login::WechatLogin;
use config::{Config, ConfigError};
use serde::Deserialize;
use std::path::PathBuf;

pub const CONFIG_FILE_NAME: &str = "config.yaml";

#[derive(Deserialize)]
pub struct Configuration {
    pub redis_session: RedisSessionConfig,
    pub server_binding: String,
    pub admin_token: String,
    pub log_file: PathBuf,
    pub wechat: WechatLogin,
    pub server_tls: Option<TlsConfig>,
    pub db_url: String,
    pub allow_origin: String,
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
