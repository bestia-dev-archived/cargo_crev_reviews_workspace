// db_review_mod.rs

//! cached data for "my reviews"
//! reading from yaml files is slow, therefore
//! it reads (sync) from "my reviews" in the background
//! only one review per crate_name_version is permitted

#![allow(dead_code)]

use lazy_static::lazy_static;
use unwrap::unwrap;

use crate::common_structs_mod::ReviewItemData;

lazy_static! {
    static ref DB_REVIEW: sled::Tree = crate::db_sled_mod::DB_SLED.open_tree(b"reviews").unwrap();
}

/// insert
pub fn insert(crate_name_version: &str, value: &ReviewItemData) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    DB_REVIEW.insert(crate_name_version, value)?;
    Ok(())
}

pub fn read(crate_name_version: &str) -> anyhow::Result<Option<ReviewItemData>> {
    let data = DB_REVIEW.get(crate_name_version)?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => Ok(None),
    }
}

pub fn all_versions_for_crate(crate_name: &str) -> anyhow::Result<Vec<ReviewItemData>> {
    let start = format!("{} 0", crate_name);
    let end = format!("{} z", crate_name);
    let mut vec = vec![];
    for x in DB_REVIEW.range(start..end) {
        let (_key, value) = x?;
        let v: ReviewItemData = serde_json::from_slice(&value)?;
        vec.push(v);
    }
    Ok(vec)
}

pub fn delete(crate_name_version: &str) {
    unwrap!(DB_REVIEW.remove(crate_name_version));
}

pub fn exists(crate_name_version: &str) -> bool {
    unwrap!(DB_REVIEW.contains_key(crate_name_version))
}

// list crate_name_version for all reviews from sled
pub fn list_all_crate_name_version() -> anyhow::Result<Vec<String>> {
    let mut vec: Vec<String> = vec![];
    for x in DB_REVIEW.iter() {
        let (key, _value) = x?;
        vec.push(std::str::from_utf8(&key).unwrap().to_string());
    }
    Ok(vec)
}
