// db_sled_mod.rs

#![allow(dead_code)]

use lazy_static::lazy_static;
use unwrap::unwrap;

use crate::{db_yanked_mod::YankedForDb, utils_mod::join_crate_version};

lazy_static! {
    /// "sled" db stays open all the time of the program running.
    /// this program checks if there is an instance already running, so to guarantee only one process access the db files.
    pub static ref DB_SLED:sled::Db = unwrap!(sled::open(unwrap!(home::home_dir()).join(".config/crev/cargo_crev_reviews_data/db")));
    /// 3 threads to download in parallel
    static ref POOL:rayon::ThreadPool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
}

/// to avoid double or triple call to crates.io for the same crate
pub fn download_in_background_crate_versions(crate_name: String) {
    POOL.spawn(move || {
        let crates_io = unwrap!(crate::crates_io_mod::crate_response(&crate_name));
        let c = crate::db_crate_mod::CrateForDb {
            crate_name: crate_name.clone(),
            description: crates_io.crate_segment.description.clone(),
        };
        unwrap!(crate::db_crate_mod::insert(&crate_name, &c));

        for crate_io_version in crates_io.versions.iter() {
            // region: VersionForDb
            let published_by_login = match &crate_io_version.published_by {
                Some(published_by) => Some(published_by.login.clone()),
                None => None,
            };
            let crate_name_version = crate::utils_mod::join_crate_version(&crate_name, &crate_io_version.num);
            let v = crate::db_version_mod::VersionForDb {
                crate_name_version: crate_name_version.clone(),
                published_by_login,
                published_date: crate_io_version.created_at.clone(),
            };
            unwrap!(crate::db_version_mod::insert(v.crate_name_version.as_str(), &v));
            // region: VersionForDb
            // store only yanked versions
            // This is ok, but not enough, because yanked can change, while all other data is immutable
            // Therefore I need also a background_sync sometimes.
            if crate_io_version.yanked {
                unwrap!(crate::db_yanked_mod::insert(
                    &crate_name_version,
                    &YankedForDb {
                        crate_name_version: crate_name_version.clone(),
                    }
                ));
            } else {
                if crate::db_yanked_mod::exists(&crate_name_version) {
                    crate::db_yanked_mod::delete(&crate_name_version);
                }
            }
            // endregion: YankedForDb
        }
    });
}

/// from cargo registry read all versions that are in db_version
/// only the yanked store in db_yanked
pub fn sync_in_background_yanked() {
    POOL.spawn(move || {
        let ns_started = crate::utils_mod::ns_start("sync_in_background_yanked");
        for x in unwrap!(crate::db_crate_mod::all_crates()).iter() {
            println!("registry crate: {}", &x.crate_name);
            for (crate_version, yanked) in unwrap!(crate::cargo_registry_mod::info_for_one_crate(&x.crate_name)).iter() {
                let crate_name_version = join_crate_version(&x.crate_name, crate_version);
                if crate::db_yanked_mod::exists(&crate_name_version) && *yanked == false {
                    crate::db_yanked_mod::delete(&crate_name_version);
                } else if !crate::db_yanked_mod::exists(&crate_name_version) && *yanked == true {
                    unwrap!(crate::db_yanked_mod::insert(
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
            let crate_name_version = &join_crate_version(&item.crate_name, &item.crate_version);
            if crate::db_review_mod::exists(crate_name_version) {
                // check the date
                let y = unwrap!(unwrap!(crate::db_review_mod::read(crate_name_version)));
                if y.date != item.date {
                    unwrap!(crate::db_review_mod::insert(crate_name_version, &item));
                }
            } else {
                unwrap!(crate::db_review_mod::insert(crate_name_version, &item));
            }
        }
        crate::utils_mod::ns_print_ms("sync_in_background_reviews", ns_started);
    });
}
