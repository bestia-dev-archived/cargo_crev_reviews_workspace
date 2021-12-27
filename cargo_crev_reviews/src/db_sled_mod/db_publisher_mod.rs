// db_publisher_mod.rs

// my trusted publishers urls
// url is better than login, because is unique

#![allow(dead_code)]

use lazy_static::lazy_static;
use unwrap::unwrap;

use crate::common_structs_mod::PublisherItemData;

lazy_static! {
    static ref DB_PUBLISHERS: sled::Tree = crate::db_sled_mod::DB_SLED.open_tree(b"publishers").unwrap();
}

pub fn insert(url: &str, value: &PublisherItemData) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    DB_PUBLISHERS.insert(url.as_bytes(), value)?;
    Ok(())
}

pub fn read(url: &str) -> anyhow::Result<Option<PublisherItemData>> {
    let data = DB_PUBLISHERS.get(url.as_bytes())?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => Ok(None),
    }
}

pub fn delete(url: &str) {
    unwrap!(DB_PUBLISHERS.remove(url.as_bytes()));
}

pub fn exists(url: &str) -> bool {
    unwrap!(DB_PUBLISHERS.contains_key(url))
}

pub fn list() -> anyhow::Result<Vec<PublisherItemData>> {
    let mut vec = vec![];
    for x in DB_PUBLISHERS.iter() {
        let (_key, value) = x?;
        let v: PublisherItemData = serde_json::from_slice(&value)?;
        vec.push(v);
    }
    Ok(vec)
}
