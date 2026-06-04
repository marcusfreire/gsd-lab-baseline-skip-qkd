use async_trait::async_trait;
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use tracing::{info, warn};

use crate::config::{StorageConfig, StorageConfigType};

pub mod memory;
pub mod sqlite;

type KeyID = uuid::Uuid;
type Key = [u8; 32];

#[derive(Debug, Clone)]
pub struct KeyData {
    pub master_sae_id: String,
    pub slave_sae_id: String,
    pub key: Key,
}

#[async_trait]
pub trait Etsi014KeyStorage: Send + Sync {
    async fn register_or_update_key(&self, key_id: KeyID, key_data: &KeyData)
    -> anyhow::Result<()>;
    async fn get_key_by_id(&self, key_id: KeyID) -> anyhow::Result<Option<KeyData>>;
    async fn delete_key(&self, key_id: KeyID) -> anyhow::Result<()>;
}

pub async fn create_storage_from_config(
    config: &StorageConfig,
) -> anyhow::Result<Box<dyn Etsi014KeyStorage>> {
    let storage: Box<dyn Etsi014KeyStorage> = match config.typ {
        StorageConfigType::Memory => {
            warn!("Using in-memory storage! All data will be lost on restart");
            Box::new(memory::MemoryKeyStorage::new())
        }
        StorageConfigType::Sqlite => {
            let path = config
                .params
                .get("path")
                .ok_or_else(|| anyhow::anyhow!("Missing path for sqlite storage"))?;
            info!("Using sqlite storage at {}", path);
            let options = SqliteConnectOptions::new()
                .filename(path)
                .create_if_missing(true);
            Box::new(
                sqlite::SqliteKeyStorage::new(
                    SqlitePoolOptions::new().connect_with(options).await?,
                )
                .await?,
            )
        }
    };

    Ok(storage)
}
