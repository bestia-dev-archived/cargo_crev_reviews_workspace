// db_sled_mod.rs

use lazy_static::lazy_static;
use unwrap::unwrap;

lazy_static! {
    /// "sled" db stays open all the time of the program running.
    /// this program checks if there is an instance already running, so to guarantee only one process access the db files.
    pub static ref DB_SLED:sled::Db = unwrap!(sled::open(unwrap!(home::home_dir()).join(".config/crev/cargo_crev_reviews_data/db_version")));
    /// 3 threads to download in parallel
    static ref POOL:rayon::ThreadPool = rayon::ThreadPoolBuilder::new().num_threads(3).build().unwrap();
}

pub fn download_in_background_crate_versions(crate_name: String) {
    POOL.spawn(move || {
        let crates_io = unwrap!(crate::crates_io_mod::crate_response(&crate_name));
        let c = crate::db_crate_mod::CrateForDb {
            crate_name: crate_name.clone(),
            description: crates_io.crate_segment.description.clone(),
        };
        unwrap!(crate::db_crate_mod::insert(&crate_name, &c));

        for crate_io_version in crates_io.versions.iter() {
            let published_by_login = match &crate_io_version.published_by {
                Some(published_by) => Some(published_by.login.clone()),
                None => None,
            };
            let crate_name_version = crate::utils_mod::join_crate_version(&crate_name, &crate_io_version.num);
            let v = crate::db_version_mod::VersionForDb {
                crate_name_version,
                published_by_login,
                published_date: crate_io_version.created_at.clone(),
            };
            unwrap!(crate::db_version_mod::insert(v.crate_name_version.as_str(), &v));
        }
    });
}
