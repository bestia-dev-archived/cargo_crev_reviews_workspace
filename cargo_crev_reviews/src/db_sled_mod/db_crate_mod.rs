// db_crate_mod.rs

#![allow(dead_code)]

use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use unwrap::unwrap;

// this struct will be cached in a local file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CrateForDb {
    pub crate_name: String,
    pub description: String,
}

lazy_static! {
    static ref DB_CRATES: sled::Tree = crate::db_sled_mod::DB_SLED.open_tree(b"crates").unwrap();
}

/// insert
pub fn insert(crate_name: &str, value: &CrateForDb) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    DB_CRATES.insert(crate_name.as_bytes(), value)?;
    Ok(())
}

pub fn read(crate_name: &str) -> anyhow::Result<Option<CrateForDb>> {
    let data = DB_CRATES.get(crate_name.as_bytes())?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => {
            // if there is no data in the database, I will GET it from crates.io in the background
            // so the next time they will be available fast.
            crate::db_sled_mod::download_in_background_crate_versions(crate_name.to_string());
            Ok(None)
        }
    }
}

pub fn delete(crate_name: &str) {
    unwrap!(DB_CRATES.remove(crate_name.as_bytes()));
}

pub fn exists(crate_name: &str) -> bool {
    unwrap!(DB_CRATES.contains_key(crate_name))
}

pub fn all_crates() -> anyhow::Result<Vec<CrateForDb>> {
    let mut vec = vec![];
    for x in DB_CRATES.iter() {
        let (_key, value) = x?;
        let v: CrateForDb = serde_json::from_slice(&value)?;
        vec.push(v);
    }
    Ok(vec)
}
