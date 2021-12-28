// db_yanked_mod.rs

//! cached list of yanked versions in the sled database
//! so the reading should be faster than reading from cargo registry index
//! this is not immutable like other information from crates.io and it needs regular updates
//! it can read the data from cargo registry.

#![allow(dead_code)]

use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use unwrap::unwrap;

// this struct will be cached in a local file
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YankedForDb {
    pub crate_name_version: String,
}

lazy_static! {
    static ref DB_YANKED: sled::Tree = crate::db_sled_mod::DB_SLED.open_tree(b"yanked").unwrap();
}

/// insert
pub fn insert(crate_name_version: &str, value: &YankedForDb) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    DB_YANKED.insert(crate_name_version.as_bytes(), value)?;
    Ok(())
}

pub fn read(crate_name_version: &str) -> anyhow::Result<Option<YankedForDb>> {
    let data = DB_YANKED.get(crate_name_version.as_bytes())?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => Ok(None),
    }
}

pub fn all_versions_for_crate(crate_name: &str) -> anyhow::Result<Vec<YankedForDb>> {
    let start = format!("{} 0", crate_name);
    let end = format!("{} z", crate_name);
    let mut vec = vec![];
    for x in DB_YANKED.range(start..end) {
        let (_key, value) = x?;
        let v: YankedForDb = serde_json::from_slice(&value)?;
        vec.push(v);
    }
    Ok(vec)
}

pub fn delete(crate_name_version: &str) {
    unwrap!(DB_YANKED.remove(crate_name_version.as_bytes()));
}

pub fn exists(crate_name_version: &str) -> bool {
    unwrap!(DB_YANKED.contains_key(crate_name_version))
}
