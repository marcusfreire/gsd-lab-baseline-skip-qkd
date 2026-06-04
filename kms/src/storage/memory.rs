use std::{collections::HashMap, sync::Mutex};

use super::*;

pub struct MemoryKeyStorage {
    // TODO: maybe use something better like dashmap
    keys: Mutex<HashMap<KeyID, KeyData>>,
}

impl MemoryKeyStorage {
    pub fn new() -> Self {
        Self {
            keys: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Etsi014KeyStorage for MemoryKeyStorage {
    async fn register_or_update_key(
        &self,
        key_id: KeyID,
        key_data: &KeyData,
    ) -> anyhow::Result<()> {
        self.keys.lock().unwrap().insert(key_id, key_data.clone());
        Ok(())
    }

    async fn get_key_by_id(&self, key_id: KeyID) -> anyhow::Result<Option<KeyData>> {
        Ok(self.keys.lock().unwrap().get(&key_id).cloned())
    }

    async fn delete_key(&self, key_id: KeyID) -> anyhow::Result<()> {
        self.keys.lock().unwrap().remove(&key_id);
        Ok(())
    }
}
