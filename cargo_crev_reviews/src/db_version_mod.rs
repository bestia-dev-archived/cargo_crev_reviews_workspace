// db_version_mod.rs

use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use unwrap::unwrap;

use crate::utils_mod::join_crate_version;
use crate::utils_mod::split_crate_version;

mod crates_io_mod;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionForDb {
    pub crate_name_version: String,
    pub yanked: bool,
    pub published_by_login: Option<String>,
    pub published_date: String,
}

lazy_static! {
    /// "sled" db for versions stays open all the time of the program running.
    /// this program checks if there is an instance already running, so to guarantee only one process access the db files.
    static ref DB_VERSION:sled::Db = unwrap!(sled::open(unwrap!(home::home_dir()).join(".config/crev/cargo_crev_reviews_data/db_version")));
    /// 3 threads to download in parallel
    static ref POOL:rayon::ThreadPool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
}

/// insert
pub fn insert(crate_name_version: &str, value: &VersionForDb) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    DB_VERSION.insert(crate_name_version.as_bytes(), value)?;
    Ok(())
}

pub fn read(crate_name_version: &str) -> anyhow::Result<Option<VersionForDb>> {
    let data = DB_VERSION.get(crate_name_version.as_bytes())?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => {
            // if there is no data in the database, I will GET it from crates.io in the background
            // so the next time they will be available fast.
            let (crate_name, _crate_version) = split_crate_version(crate_name_version);
            download_in_background(crate_name);
            Ok(None)
        }
    }
}

pub fn download_in_background(crate_name: String) {
    POOL.spawn(move || {
        let versions = unwrap!(crates_io_mod::get_vec_of_versions(&crate_name));
        for crate_io_version in versions.iter() {
            let published_by_login = match &crate_io_version.published_by {
                Some(published_by) => Some(published_by.login.clone()),
                None => None,
            };
            let crate_name_version = join_crate_version(&crate_name, &crate_io_version.num);
            let v = VersionForDb {
                crate_name_version,
                yanked: crate_io_version.yanked,
                published_by_login,
                published_date: crate_io_version.created_at.clone(),
            };
            unwrap!(insert(v.crate_name_version.as_str(), &v));
        }
    });
}

pub fn delete(crate_name_version: &str) {
    unwrap!(DB_VERSION.remove(crate_name_version.as_bytes()));
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
        download_in_background(crate_name.to_string());
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
