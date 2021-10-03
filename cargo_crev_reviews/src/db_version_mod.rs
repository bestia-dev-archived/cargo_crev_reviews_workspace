// db_version_mod.rs

use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use unwrap::unwrap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionForDb {
    pub crate_name_version: String,
    pub yanked: bool,
    pub published_by_login: Option<String>,
    pub published_date: String,
}

lazy_static! {
    /// sled db for versions stays open all the time of the program running. Only for one process.
    static ref VERSION_DB:sled::Db = unwrap!(sled::open(unwrap!(home::home_dir()).join(".config/crev/cargo_crev_reviews_versions.sled")));
}

/// insert
pub fn insert(crate_name_version: &str, value: &VersionForDb) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    VERSION_DB.insert(crate_name_version.as_bytes(), value)?;
    Ok(())
}

pub fn read(crate_name_version: &str) -> anyhow::Result<Option<VersionForDb>> {
    let data = VERSION_DB.get(crate_name_version.as_bytes())?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => Ok(None),
    }
}

pub fn delete(crate_name_version: &str) {
    unwrap!(VERSION_DB.remove(crate_name_version.as_bytes()));
}

pub fn all_versions_for_crate(crate_name: &str) -> anyhow::Result<Vec<VersionForDb>> {
    let start = format!("{} 0", crate_name);
    let end = format!("{} z", crate_name);
    let mut vec = vec![];
    for x in VERSION_DB.range(start..end) {
        let (_key, value) = x?;
        let v: VersionForDb = serde_json::from_slice(&value)?;
        vec.push(v);
    }
    Ok(vec)
}

pub fn exists(crate_name: &str) -> bool {
    let start = format!("{} 0", crate_name);
    let end = format!("{} z", crate_name);
    match VERSION_DB.range(start..end).next() {
        Some(_) => true,
        None => false,
    }
}
