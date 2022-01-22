// db_version_mod.rs

//! cached data for crate versions
//! for needed crates it reads (sync) from crates.io api in the background

#![allow(dead_code)]

use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use unwrap::unwrap;

use crate::utils_mod::crate_version_split;

// this struct will be cached in a local file
// most of the data on crates.io is immutable and they are easy to cache
// but yanked can change any time. Therefore yanked is not a good field for this db
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionForDb {
    pub crate_name_version: String,
    pub published_by_url: Option<String>,
    pub published_date: String,
}

lazy_static! {
    static ref DB_VERSION: sled::Tree = crate::db_sled_mod::DB_SLED.open_tree(b"versions").unwrap();
}

/// insert
pub fn insert(crate_name_version: &str, value: &VersionForDb) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    DB_VERSION.insert(crate_name_version, value)?;
    Ok(())
}

pub fn read(crate_name_version: &str) -> anyhow::Result<Option<VersionForDb>> {
    let data = DB_VERSION.get(crate_name_version)?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => {
            // if there is no data in the database, I will GET it from crates.io in the background
            // so the next time they will be available fast.
            let (crate_name, _crate_version) = crate_version_split(crate_name_version);
            crate::db_sled_mod::download_in_background_crate_versions(crate_name);
            Ok(None)
        }
    }
}

pub fn delete(crate_name_version: &str) {
    unwrap!(DB_VERSION.remove(crate_name_version));
}

pub fn all_versions_for_crate(crate_name: &str) -> anyhow::Result<Vec<VersionForDb>> {
    let start = format!("{} 0", crate_name);
    let end = format!("{} z", crate_name);
    let mut vec = vec![];
    for x in DB_VERSION.range(start..end) {
        let (_key, value) = x?;
        let v: VersionForDb = serde_json::from_slice(&value)?;
        vec.push(v);
    }
    if vec.is_empty() {
        // if there is no data in the database, I will GET it from crates.io in the background
        // so the next time they will be available fast.
        crate::db_sled_mod::download_in_background_crate_versions(crate_name.to_string());
    }
    Ok(vec)
}

pub fn exists(crate_name: &str) -> bool {
    let start = format!("{} 0", crate_name);
    let end = format!("{} z", crate_name);
    match DB_VERSION.range(start..end).next() {
        Some(_) => true,
        None => false,
    }
}

pub fn clear() {
    unwrap!(DB_VERSION.clear());
}
