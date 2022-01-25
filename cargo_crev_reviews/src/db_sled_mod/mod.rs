// db_sled_mod.rs

//! db_sled is a database that works in memory, but is persisted on disk
//! it looks very fast. It must be used uniquely by only one process!
//! Be careful when changing the shape of the data: then there must exist a migration program to upgrade the data store

#![allow(dead_code)]

pub mod db_crate_mod;
pub mod db_metadata_mod;
pub mod db_publisher_mod;
pub mod db_review_mod;
pub mod db_verify_mod;
pub mod db_version_mod;
pub mod db_yanked_mod;

use lazy_static::lazy_static;
use unwrap::unwrap;

use crate::{db_sled_mod::db_yanked_mod::YankedForDb, utils_mod::crate_version_join};

lazy_static! {
    /// "sled" db stays open all the time of the program running.
    /// this program on start checks if there is an instance already running, so to guarantee only one process access the db files.
    pub static ref DB_SLED:sled::Db = unwrap!(sled::open(crate::CARGO_CREV_REVIEWS_SLED_DB.as_path()));
    /// 3 threads to download in parallel
    static ref POOL:rayon::ThreadPool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
}

/// to avoid double or triple call to crates.io for the same crate
pub fn download_in_background_crate_versions(crate_name: String) {
    POOL.spawn(move || {
        let ns_started = crate::utils_mod::ns_start(&format!("download_in_background_crate_versions {}", &crate_name));
        // cargo_crev_reviews_wasm is not on crates.io
        match crate::crates_io_mod::crate_response(&crate_name) {
            Err(_err) => log::info!("crate {} is not on crates.io.", &crate_name),
            Ok(crates_io) => {
                let c = crate::db_sled_mod::db_crate_mod::CrateForDb {
                    crate_name: crate_name.clone(),
                    description: crates_io.crate_segment.description.clone(),
                };
                unwrap!(crate::db_sled_mod::db_crate_mod::insert(&crate_name, &c));

                for crate_io_version in crates_io.versions.iter() {
                    // region: VersionForDb
                    let published_by_url = match &crate_io_version.published_by {
                        Some(published_by) => Some(published_by.url.clone()),
                        None => None,
                    };
                    let crate_name_version = crate::utils_mod::crate_version_join(&crate_name, &crate_io_version.num);
                    let v = crate::db_sled_mod::db_version_mod::VersionForDb {
                        crate_name_version: crate_name_version.clone(),
                        published_by_url,
                        published_date: crate_io_version.created_at.clone(),
                    };
                    unwrap!(crate::db_sled_mod::db_version_mod::insert(v.crate_name_version.as_str(), &v));
                    // region: VersionForDb
                    // store only yanked versions
                    // This is ok, but not enough, because yanked can change, while all other data is immutable
                    // Therefore I need also a background_sync sometimes.
                    if crate_io_version.yanked {
                        unwrap!(crate::db_sled_mod::db_yanked_mod::insert(
                            &crate_name_version,
                            &YankedForDb {
                                crate_name_version: crate_name_version.clone(),
                            }
                        ));
                    } else {
                        if crate::db_sled_mod::db_yanked_mod::exists(&crate_name_version) {
                            crate::db_sled_mod::db_yanked_mod::delete(&crate_name_version);
                        }
                    }
                    // endregion: YankedForDb
                }
            }
        }
        crate::utils_mod::ns_print_ms("download_in_background_crate_versions", ns_started);
    });
}

/// from cargo registry read all versions that are in db_version
/// only the yanked store in db_yanked
pub fn sync_in_background_yanked() {
    POOL.spawn(move || {
        let ns_started = crate::utils_mod::ns_start("sync_in_background_yanked");
        for x in unwrap!(crate::db_sled_mod::db_crate_mod::all_crates()).iter() {
            // println!("registry crate: {}", &x.crate_name);
            for (crate_version, yanked) in unwrap!(crate::cargo_registry_mod::info_for_one_crate(&x.crate_name)).iter() {
                let crate_name_version = crate_version_join(&x.crate_name, crate_version);
                if crate::db_sled_mod::db_yanked_mod::exists(&crate_name_version) && *yanked == false {
                    crate::db_sled_mod::db_yanked_mod::delete(&crate_name_version);
                } else if !crate::db_sled_mod::db_yanked_mod::exists(&crate_name_version) && *yanked == true {
                    unwrap!(crate::db_sled_mod::db_yanked_mod::insert(
                        &crate_name_version,
                        &YankedForDb {
                            crate_name_version: crate_name_version.clone(),
                        }
                    ));
                }
            }
        }
        crate::utils_mod::ns_print_ms("sync_in_background_yanked", ns_started);
    });
}

/// working with crev data looks slow.
/// I will sync in background with fast sled database and work from there.
pub fn sync_in_background_reviews() {
    POOL.spawn(move || {
        let ns_started = crate::utils_mod::ns_start("sync_in_background_reviews");
        let vec = unwrap!(crate::crev_mod::crev_list_my_reviews(&None));
        for x in vec.iter() {
            let item = crate::utils_mod::from_crev_to_item(x);
            let crate_name_version = &crate_version_join(&item.crate_name, &item.crate_version);
            if crate::db_sled_mod::db_review_mod::exists(crate_name_version) {
                // check the date
                let y = unwrap!(unwrap!(crate::db_sled_mod::db_review_mod::read(crate_name_version)));
                if y.date != item.date {
                    unwrap!(crate::db_sled_mod::db_review_mod::insert(crate_name_version, &item));
                }
            } else {
                unwrap!(crate::db_sled_mod::db_review_mod::insert(crate_name_version, &item));
            }
        }
        crate::utils_mod::ns_print_ms("sync_in_background_reviews", ns_started);
    });
}

/// working with crev verify looks slow.
/// I will sync in background with fast sled database and work from there.
pub fn sync_in_background_verify() {
    POOL.spawn(move || {
        let ns_started = crate::utils_mod::ns_start("sync_in_background_verify");
        let verify_list_data = unwrap!(crate::crev_mod::verify_project());
        for item in verify_list_data.list_of_verify.iter() {
            let crate_name_version = &crate_version_join(&item.crate_name, &item.crate_version);
            // always insert/update
            unwrap!(crate::db_sled_mod::db_verify_mod::insert(crate_name_version, &item));
        }
        crate::utils_mod::ns_print_ms("sync_in_background_verify", ns_started);
    });
}

/// if the data structure changes then it must be updated by a migration
pub fn db_sled_migration_update(cargo_pkg_version: &str) {
    let pkg_semver = unwrap!(semver::Version::parse(&cargo_pkg_version));
    let db_version = unwrap!(db_metadata_mod::get_version());
    let db_semver = unwrap!(semver::Version::parse(&db_version));
    // log::info!("db_sled_migration_update pkg={}, db={}", &pkg_semver, &db_semver);

    upgrade_to_2021_1228_1255(&pkg_semver, &db_semver);
    upgrade_to_2021_1228_1528(&pkg_semver, &db_semver);
}

fn upgrade_to_2021_1228_1255(pkg_semver: &crev_data::Version, db_semver: &crev_data::Version) {
    let db_updated_semver = unwrap!(semver::Version::parse("2021.1228.1255"));

    if db_updated_semver.cmp(&pkg_semver) == std::cmp::Ordering::Less && db_semver.cmp(&db_updated_semver) == std::cmp::Ordering::Less {
        log::info!("migrating db_sled from {} to {}", &db_semver, &db_updated_semver);
        // clear the content of db_version because published_by_login changed to published_by_url
        // this is only a data cache tree and will be populated automatically
        db_version_mod::clear();

        db_metadata_mod::set_version(&db_updated_semver.to_string());
    }
}

fn upgrade_to_2021_1228_1528(pkg_semver: &crev_data::Version, db_semver: &crev_data::Version) {
    let db_updated_semver = unwrap!(semver::Version::parse("2021.1228.1528"));

    if db_updated_semver.cmp(&pkg_semver) == std::cmp::Ordering::Less && db_semver.cmp(&db_updated_semver) == std::cmp::Ordering::Less {
        log::info!("migrating db_sled from {} to {}", &db_semver, &db_updated_semver);

        // init with some default trusted publishers
        db_publisher_mod::init_with_some_default();

        db_metadata_mod::set_version(&db_updated_semver.to_string());
    }
}
