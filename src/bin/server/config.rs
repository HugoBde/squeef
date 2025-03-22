use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::Deserialize;
use serde_inline_default::serde_inline_default;

lazy_static! {
    pub static ref CONFIG: Config = {
        let config = std::fs::read_to_string("squeef.toml").unwrap();
        toml::from_str(&config).unwrap()
    };
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,

    #[serde(default)]
    pub storage: StorageConfig,
}

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    #[serde_inline_default(6870)]
    pub port: u16,

    #[serde_inline_default(8)]
    pub max_concurrent_connection: isize,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            port: 6870,
            max_concurrent_connection: 8,
        }
    }
}

#[serde_inline_default]
#[derive(Deserialize, Debug)]
pub struct StorageConfig {
    #[serde_inline_default(PathBuf::from("/var/lib/squeef"))]
    pub persistent_storage_dir: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> StorageConfig {
        StorageConfig {
            persistent_storage_dir: PathBuf::from("/var/lib/squeef"),
        }
    }
}
