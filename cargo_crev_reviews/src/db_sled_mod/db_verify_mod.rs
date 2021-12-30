// db_verify_mod.rs

//! cached data for the command "verify"
//! the command can be slow therefore
//! it reads (sync) the result of the command in the background

#![allow(dead_code)]

use lazy_static::lazy_static;
use unwrap::unwrap;

use crate::common_structs_mod::VerifyItemData;

lazy_static! {
    static ref DB_VERIFY: sled::Tree = crate::db_sled_mod::DB_SLED.open_tree(b"verify").unwrap();
}

/// insert
pub fn insert(crate_name_version: &str, value: &VerifyItemData) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    DB_VERIFY.insert(crate_name_version, value)?;
    Ok(())
}

pub fn read(crate_name_version: &str) -> anyhow::Result<Option<VerifyItemData>> {
    let data = DB_VERIFY.get(crate_name_version)?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => Ok(None),
    }
}

pub fn delete(crate_name_version: &str) {
    unwrap!(DB_VERIFY.remove(crate_name_version));
}

pub fn exists(crate_name_version: &str) -> bool {
    unwrap!(DB_VERIFY.contains_key(crate_name_version))
}
