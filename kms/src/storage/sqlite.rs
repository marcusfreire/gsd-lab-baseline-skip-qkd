use async_trait::async_trait;
use sqlx::Row;

use super::{Etsi014KeyStorage, KeyData};

pub struct SqliteKeyStorage {
    pool: sqlx::SqlitePool,
}

impl SqliteKeyStorage {
    pub async fn new(pool: sqlx::SqlitePool) -> anyhow::Result<Self> {
        let storage = Self { pool };
        storage.init().await?;
        Ok(storage)
    }

    async fn init(&self) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS keys (
                key_id TEXT PRIMARY KEY,
                master_sae_id TEXT NOT NULL,
                slave_sae_id TEXT NOT NULL,
                key BLOB NOT NULL
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
impl Etsi014KeyStorage for SqliteKeyStorage {
    async fn register_or_update_key(
        &self,
        key_id: super::KeyID,
        key_data: &super::KeyData,
    ) -> anyhow::Result<()> {
        let key_id_str = key_id.to_string();
        let key = key_data.key.as_slice();
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO keys (key_id, master_sae_id, slave_sae_id, key)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(key_id_str)
        .bind(&key_data.master_sae_id)
        .bind(&key_data.slave_sae_id)
        .bind(key)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_key_by_id(&self, key_id: super::KeyID) -> anyhow::Result<Option<super::KeyData>> {
        let key_id_str = key_id.to_string();
        let row = sqlx::query(
            r#"
            SELECT key_id, master_sae_id, slave_sae_id, key
            FROM keys
            WHERE key_id = ?
            "#,
        )
        .bind(key_id_str)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|row| KeyData {
            master_sae_id: row.get("master_sae_id"),
            slave_sae_id: row.get("slave_sae_id"),
            key: row
                .get::<Vec<u8>, _>("key")
                .try_into()
                .expect("Key has wrong size"),
        }))
    }

    async fn delete_key(&self, key_id: super::KeyID) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM keys
            WHERE key_id = ?
            "#,
        )
        .bind(key_id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
