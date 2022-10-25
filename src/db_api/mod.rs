pub mod ducks;
pub mod exhibits;
pub mod locations;
pub mod public;
pub mod rankings;

use crate::prisma::{new_client_with_url, PrismaClient};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
pub struct DB(Arc<PrismaClient>);

impl DB {
    pub async fn new(url: &str) -> anyhow::Result<Self> {
        Ok(DB(Arc::new(new_client_with_url(url).await?)))
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bilingual {
    en: String,
    cn: String,
}
