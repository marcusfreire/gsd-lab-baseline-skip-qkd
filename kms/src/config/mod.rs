use std::{collections::HashMap, path::Path};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub listen: String,
    #[serde(rename = "use_mTLS")]
    pub use_mtls: bool,
    pub storage: StorageConfig,
    pub tls: Option<TlsConfig>,
    pub kme_topology: HashMap<String, KmePeerConfig>,
}

#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    #[serde(rename = "type")]
    pub typ: StorageConfigType,
    pub params: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub enum StorageConfigType {
    #[serde(rename = "memory")]
    Memory,
    #[serde(rename = "sqlite")]
    Sqlite,
}

#[derive(Debug, Deserialize)]
pub struct KmePeerConfig {
    pub connected_saes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct TlsConfig {
    pub server_cert_path: String,
    pub server_key_path: String,
    pub client_ca_cert_path: String,
}

impl Config {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        serde_yaml::from_reader(reader).map_err(|e| anyhow::anyhow!(e))
    }
}
