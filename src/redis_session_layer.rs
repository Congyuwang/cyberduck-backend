use crate::SERVER_CONFIG;
use anyhow::Result;
use axum_database_sessions::{
    AxumRedisPool, AxumRedisSessionStore, AxumSessionConfig, AxumSessionLayer, Key, SameSite,
};
use serde::Deserialize;
use time::Duration;

#[derive(Deserialize)]
pub struct RedisSessionConfig {
    session_secret: String,
    session_expiration: i64,
    cookie_name: String,
    redis_url: String,
}

impl RedisSessionConfig {
    pub async fn build_layer(&self) -> Result<AxumSessionLayer<AxumRedisPool>> {
        let redis = redis::Client::open(self.redis_url.as_str())?;

        let session_secret = base64::decode(&self.session_secret)?;

        let session_config = AxumSessionConfig::default()
            .with_cookie_name(&SERVER_CONFIG.redis_session.cookie_name)
            .with_always_save(false)
            .with_cookie_same_site(SameSite::Lax)
            .with_http_only(true)
            .with_lifetime(Duration::seconds(self.session_expiration))
            .with_secure(true)
            .with_key(Key::from(session_secret.as_slice()));

        let session_store =
            AxumRedisSessionStore::new(Some(AxumRedisPool::from(redis)), session_config);
        session_store.initiate().await?;
        Ok(AxumSessionLayer::new(session_store))
    }
}
