// db_metadata_mod.rs

//! persistent storage for metadata( version, config,...)

#![allow(dead_code)]

use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use unwrap::unwrap;

use crate::common_structs_mod::ConfigData;

// this struct will be cached in a local file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrateForDb {
    pub crate_name: String,
    pub description: String,
}

lazy_static! {
    /// I will store the "version" string, so I can use it for migration upgrades
    /// and also the "config" struct
    static ref DB_SLED_METADATA: sled::Tree = crate::db_sled_mod::DB_SLED.open_tree(b"metadata").unwrap();
}

pub fn get_version() -> anyhow::Result<String> {
    let version_opt = DB_SLED_METADATA.get("version")?;
    let db_version: String = match version_opt {
        Some(data) => String::from_utf8_lossy(data.as_ref()).to_string(),
        None => "0.0.0".to_string(),
    };
    // return
    Ok(db_version)
}

pub fn set_version(version: &str) {
    unwrap!(DB_SLED_METADATA.insert("version", version));
}

pub fn get_config() -> anyhow::Result<ConfigData> {
    let config_opt = DB_SLED_METADATA.get("config")?;
    let db_config: ConfigData = match config_opt {
        Some(value) => serde_json::from_slice(&value)?,
        None => ConfigData {
            // defaults for config
            code_editor_path: "/usr/bin/code".to_string(),
            browser_path: "/usr/bin/xdg-open".to_string(),
        },
    };
    // return
    Ok(db_config)
}

pub fn set_config(config: ConfigData) {
    let config = unwrap!(serde_json::to_vec(&config));
    unwrap!(DB_SLED_METADATA.insert("config", config));
}
