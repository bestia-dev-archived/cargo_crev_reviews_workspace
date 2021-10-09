// crev_mod.rs

//! functions around the crev data and lib

// crev-lib="0.22.0"
// crev-data="0.22.0"
// anyhow="1.0.43"
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0.66"
// fnv="1.0.7"
// lazy_static="1.4.0"
// unwrap="1.2.1"
// serde_yaml = "0.8.20"
// home="0.5.3"

#![allow(dead_code)]

use anyhow::Context;
use crev_data::{
    proof::{CommonOps, ContentExt},
    Level, Rating,
};
use crev_lib::ProofStore;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use std::{env, sync::Mutex};
use std::{ops::Range, str::FromStr, vec};
use unwrap::unwrap;

use crate::common_structs_mod::*;
use crate::stdio_input_password_mod;
use crate::utils_mod::*;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with async closures
    static ref CREV_UNLOCKED: Mutex<Option<crev_data::id::UnlockedId>>=Mutex::new(None);
    static ref CREV_LOCAL: Mutex<Option<crev_lib::Local>>=Mutex::new(None);
}

#[derive(Deserialize, Clone, Default)]
pub struct PackageSegment {
    pub name: String,
    pub version: String,
    pub version_for_sorting: Option<String>,
}

#[derive(Deserialize, Clone, Default)]
pub struct ReviewSegment {
    pub thoroughness: Level,
    pub understanding: Level,
    pub rating: Rating,
}

/// only the fields I care about for Review
#[derive(Deserialize, Clone)]
pub struct ProofCrevForReview {
    pub date: String,
    pub package: PackageSegment,
    pub review: Option<ReviewSegment>,
    pub comment: Option<String>,
}

// region: copied from cargo-crev  (maybe should add this to crev-lib?)
pub const VCS_INFO_JSON_FILE: &str = ".cargo_vcs_info.json";

#[derive(Debug, Clone, Deserialize)]
pub struct VcsInfoJson {
    git: VcsInfoJsonGit,
}

#[derive(Debug, Clone, Deserialize)]
pub enum VcsInfoJsonGit {
    #[serde(rename = "sha1")]
    Sha1(String),
}

impl VcsInfoJson {
    fn read_from_crate_dir(pkg_dir: &std::path::Path) -> anyhow::Result<Option<Self>> {
        let path = pkg_dir.join(VCS_INFO_JSON_FILE);

        if path.exists() {
            let txt = std::fs::read_to_string(&path)?;
            let info: VcsInfoJson = serde_json::from_str(&txt)?;
            Ok(Some(info))
        } else {
            Ok(None)
        }
    }
    fn get_git_revision(&self) -> Option<String> {
        let VcsInfoJsonGit::Sha1(ref s) = self.git;
        Some(s.to_string())
    }
}

pub fn cargo_min_ignore_list() -> fnv::FnvHashSet<std::path::PathBuf> {
    let mut ignore_list = std::collections::HashSet::default();
    ignore_list.insert(std::path::PathBuf::from(".cargo-ok"));
    ignore_list
}

pub fn vcs_info_to_revision_string(vcs: Option<VcsInfoJson>) -> String {
    vcs.and_then(|vcs| vcs.get_git_revision()).unwrap_or_else(|| "".into())
}

// endregion: copied from cargo-crev  (maybe missing in cargo lib)

/// unlock crev_id interactively
pub fn unlock_crev_id_interactively() -> anyhow::Result<()> {
    let crev_local = crev_lib::local::Local::auto_create_or_open()?;
    let crev_unlocked = crev_local.read_current_unlocked_id(&stdio_input_password_mod::read_passphrase_interactively)?;
    println!("Unlocked.");

    // write to static mut
    *CREV_UNLOCKED.lock().unwrap() = Some(crev_unlocked);
    *CREV_LOCAL.lock().unwrap() = Some(crev_local);

    // return
    Ok(())
}

/// list my reviews
pub fn crev_list_my_reviews(filter: &Option<ReviewFilterData>) -> anyhow::Result<Vec<ProofCrevForReview>> {
    let mut vec_proof: Vec<ProofCrevForReview> = vec![];
    // open every *.proof.crev file in my crev reviews directory
    for path in proof_crev_files_paths()?.iter() {
        let file_content = std::fs::read_to_string(path)?;
        let mut pos_cursor = 0;
        loop {
            let range = find_range_including_delimiters(&file_content, &mut pos_cursor, "----- BEGIN CREV PROOF -----", "----- END CREV PROOF -----");
            match range {
                Some(mut range) => {
                    // if there is some white space after the segment,include it in the range.
                    let pos_2 = find_pos_before_delimiter(&file_content, pos_cursor, "----- BEGIN CREV PROOF -----");
                    range.end = match pos_2 {
                        Some(pos_2) => pos_2,
                        None => file_content.len(),
                    };

                    let proof_text = unwrap!(file_content.get(range.clone()));
                    // This must not panic because it is internal to the previous range.
                    let range_yaml = unwrap!(find_range_between_delimiters(
                        proof_text,
                        &mut 0,
                        "----- BEGIN CREV PROOF -----",
                        "----- SIGN CREV PROOF -----",
                    ));
                    // if this panics it's a bug in the code and not an exception to handle
                    let yaml = unwrap!(proof_text.get(range_yaml));
                    let proof_crev: ProofCrevForReview = unwrap!(serde_yaml::from_str(yaml));
                    // push it to vector, if it filters
                    match filter {
                        // no filter, push all for list
                        None => vec_proof.push(proof_crev),
                        Some(filter) => {
                            // always filtered at least by crate_name
                            if filter.crate_name == proof_crev.package.name {
                                match &filter.crate_version {
                                    None => {
                                        // all the versions of one crate
                                        vec_proof.push(proof_crev);
                                    }
                                    Some(version) => {
                                        if version == proof_crev.package.version.as_str() {
                                            // exact match
                                            vec_proof.push(proof_crev);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                None => break,
            }
        }
    }
    // return
    Ok(vec_proof)
}

pub fn crev_sort_list_by_name_version(vec_of_reviews: &mut Vec<ProofCrevForReview>) {
    vec_of_reviews.sort_by(|a, b| match a.package.name.cmp(&b.package.name) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => {
            let a = semver::Version::parse(&a.package.version).unwrap();
            let b = semver::Version::parse(&b.package.version).unwrap();
            a.cmp(&b)
        }
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });
}

/// edit my reviews, find it in list
pub fn crev_edit_review(filter: ReviewFilterData) -> anyhow::Result<ProofCrevForReview> {
    let vec = crev_list_my_reviews(&Some(filter))?;
    if vec.is_empty() {
        anyhow::bail!("Crate version not found in my reviews!");
    }
    // return
    Ok(vec[0].clone())
}

/// edit or new
pub fn crev_edit_or_new_review(filter: ReviewFilterData) -> anyhow::Result<ProofCrevForReview> {
    let new_filter = ReviewFilterData {
        crate_name: filter.crate_name.clone(),
        crate_version: None,
        old_crate_version: None,
    };

    let mut vec_of_reviews = crev_list_my_reviews(&Some(new_filter))?;
    if vec_of_reviews.is_empty() {
        anyhow::bail!("Crate reviews for {} not found in my reviews!", filter.crate_name.as_str());
    }
    crev_sort_list_by_name_version(&mut vec_of_reviews);
    let mut review = vec_of_reviews.last().context("last")?.clone();
    review.package.version = filter.crate_version.context("none version")?.clone();

    // return
    Ok(review)
}

/// list the user-selected crate+version
/// then find the newest version in cargo registry index and change the version field
/// all the rest stays the same
pub fn crev_new_version(filter: ReviewFilterData) -> anyhow::Result<ProofCrevForReview> {
    let new_filter = ReviewFilterData {
        crate_name: filter.crate_name.clone(),
        crate_version: None,
        old_crate_version: None,
    };
    let vec_of_reviews = crev_list_my_reviews(&Some(new_filter))?;
    if vec_of_reviews.is_empty() {
        anyhow::bail!("Crate reviews for {} not found in my reviews!", filter.crate_name.as_str());
    }
    let max_version = crate::cargo_registry_mod::max_version_from_registry_index(filter.crate_name.as_str())?;

    if let Some(version) = filter.crate_version.as_ref() {
        if version == max_version.as_str() {
            anyhow::bail!("Max version {} is already reviewed!", &max_version);
        }
        let path_dir = crate::cargo_registry_mod::cargo_registry_src_dir_for_crate(&filter.crate_name, &max_version)?;
        if !path_dir.exists() {
            anyhow::bail!("Max version {} src is not cached on your system. It means you don't have a dependency on it in your projects. You cannot review a crate version that you don't use.", &max_version);
        }
        for x in vec_of_reviews.iter() {
            if x.package.version.as_str() == version {
                let mut review = x.clone();
                review.package.version = max_version;
                return Ok(review);
            }
        }
    }

    // return last review, or default
    anyhow::bail!("Cannot add new version for {}!", &filter.crate_name);
}

/// create save review proof
pub fn crev_save_review(
    crate_name: &str,
    crate_version_str: &str,
    thoroughness: crev_data::Level,
    understanding: crev_data::Level,
    rating: crev_data::Rating,
    comment_md: &str,
) -> anyhow::Result<()> {
    let review = crev_data::proof::Review {
        thoroughness,
        understanding,
        rating,
    };
    let crate_root = crate::cargo_registry_mod::cargo_registry_src_dir_for_crate(crate_name, crate_version_str)?;

    if !crate_root.exists() {
        anyhow::bail!("The crate {}-{} does not exist in the local cargo registry cache. You must use the crate as dependency in your projects, if you want to review it. This way cargo will download the source code for the crate that you review. ", crate_name, crate_version_str);
    }
    let digest_clean = crev_lib::get_recursive_digest_for_dir(&crate_root, &cargo_min_ignore_list())?;
    let vcs = VcsInfoJson::read_from_crate_dir(&crate_root)?;

    let crate_version = crev_data::Version::from_str(crate_version_str)?;

    // new reviews only for crates that come from crates.io
    let package_id = crev_data::proof::PackageVersionId::new("https://crates.io".to_string(), crate_name.to_string(), crate_version);

    let package_info = crev_data::proof::PackageInfo {
        id: package_id,
        digest: digest_clean.into_vec(),
        digest_type: crev_data::proof::default_digest_type(),
        revision: vcs_info_to_revision_string(vcs),
        revision_type: crev_data::proof::default_revision_type(),
    };

    let proof =
        CREV_UNLOCKED
            .lock()
            .unwrap()
            .as_ref()
            .unwrap()
            .as_public_id()
            .create_package_review_proof(package_info.clone(), review, comment_md.to_string())?;

    // for `sign_by()` to work there must be `use crev_data::proof::ContentExt;`
    let proof = proof.sign_by(&CREV_UNLOCKED.lock().unwrap().as_ref().unwrap())?;

    // if exists an old proof with same crate+version, delete it and then save the new one
    delete_review_proofs(crate_name, crate_version_str)?;

    // it needs `use crev_lib::ProofStore;`
    CREV_LOCAL.lock().unwrap().as_ref().unwrap().insert(&proof)?;

    let commit_msg = format!("Add review for {} v{}", crate_name, crate_version_str);
    CREV_LOCAL.lock().unwrap().as_ref().unwrap().proof_dir_commit(&commit_msg)?;

    // return
    Ok(())
}

pub fn rating_parse(rating: &str) -> anyhow::Result<Rating> {
    match rating.to_lowercase().as_str() {
        "negative" => Ok(Rating::Negative),
        "neutral" => Ok(Rating::Neutral),
        "positive" => Ok(Rating::Positive),
        "strong" => Ok(Rating::Strong),
        _ => anyhow::bail!("unrecognized rating: {}", rating),
    }
}
pub fn rating_to_string(rating: &Rating) -> String {
    match rating {
        Rating::Negative => "negative".to_string(),
        Rating::Neutral => "neutral".to_string(),
        Rating::Positive => "positive".to_string(),
        Rating::Strong => "strong".to_string(),
    }
}

// verify the signature of a proof
pub fn _verify_proof(yaml: &str) -> anyhow::Result<()> {
    let proofs = crev_data::proof::Proof::parse_from(yaml.as_bytes())?;
    let proof = &proofs[0];
    println!("signature: {}", proof.signature());
    println!("id: {}", proof.author_public_id().id);
    println!("body:\n{}", proof.body());
    proof.verify()?;

    println!("Proof verified ok.");
    // return
    Ok(())
}

pub fn reviews_dir() -> anyhow::Result<String> {
    let path = CREV_LOCAL.lock().unwrap().as_ref().unwrap().get_proofs_dir_path()?;
    let crev_id = CREV_LOCAL.lock().unwrap().as_ref().unwrap().read_current_id()?;
    Ok(format!("{}/{}/reviews", path.to_str().unwrap(), crev_id))
}

fn proof_crev_files_paths() -> anyhow::Result<Vec<String>> {
    let reviews_dir = reviews_dir()?;
    let mut v = Vec::new();
    for entry in std::fs::read_dir(&reviews_dir)? {
        let entry = entry?;
        let path = entry.path();
        let str_path = path.to_str().unwrap();
        if str_path.ends_with(".proof.crev") {
            v.push(str_path.to_string());
        }
    }
    Ok(v)
}

/// remove old proofs, so the new review proof will be unique
pub fn delete_review_proofs(crate_name: &str, crate_version: &str) -> anyhow::Result<()> {
    // open every *.proof.crev file in my crev reviews directory
    for path in proof_crev_files_paths()?.iter() {
        let mut file_content = std::fs::read_to_string(path)?;
        let mut pos_cursor = 0;
        let mut vec_range: Vec<Range<usize>> = vec![];
        loop {
            let range = find_range_including_delimiters(&file_content, &mut pos_cursor, "----- BEGIN CREV PROOF -----", "----- END CREV PROOF -----");
            match range {
                Some(mut range) => {
                    // if there is some white space after the segment,include it in the range.
                    let pos_2 = find_pos_before_delimiter(&file_content, pos_cursor, "----- BEGIN CREV PROOF -----");
                    range.end = match pos_2 {
                        Some(pos_2) => pos_2,
                        None => file_content.len(),
                    };

                    let proof_text = unwrap!(file_content.get(range.clone()));
                    // This must not panic because it is internal to the previous range.
                    let range_yaml = unwrap!(find_range_between_delimiters(
                        proof_text,
                        &mut 0,
                        "----- BEGIN CREV PROOF -----",
                        "----- SIGN CREV PROOF -----",
                    ));
                    // if this panics it's a bug in the code and not an exception to handle
                    let yaml = unwrap!(proof_text.get(range_yaml));
                    let proof_crev: ProofCrevForReview = unwrap!(serde_yaml::from_str(yaml));
                    // I will remove only if it has the review segment.
                    if proof_crev.review.is_some() && proof_crev.package.name == crate_name && proof_crev.package.version == crate_version {
                        vec_range.push(range.clone());
                    }
                }
                None => break,
            }
        }
        if !vec_range.is_empty() {
            // remove all ranges from the bottom up
            while let Some(range) = vec_range.pop() {
                file_content.drain(range);
            }
            if file_content.trim().is_empty() {
                // delete the file
                unwrap!(std::fs::remove_file(path));
            } else {
                std::fs::write(path, file_content)?;
            }
        }
    }
    Ok(())
}

pub fn crev_publish() -> anyhow::Result<String> {
    let output = std::process::Command::new("cargo").arg("crev").arg("publish").output()?;
    let ret_val = format!("{} {}", String::from_utf8(output.stdout)?, String::from_utf8(output.stderr)?);
    Ok(ret_val)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrustedPublisherDataFile {
    trusted_publishers: Vec<TrustedPublisher>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TrustedPublisher {
    pub login: String,
}

/// verify_project should return some data quickly, but in the background start to fill the db_version
/// for all these crates. So the next time we have more complete data
pub fn verify_project() -> anyhow::Result<VerifyListData> {
    let ns_started = crate::utils_mod::ns_start("verify_project");
    crate::db_sled_mod::sync_in_background_reviews();
    crate::db_sled_mod::sync_in_background_yanked();

    let output = std::process::Command::new("cargo").arg("crev").arg("verify").output().unwrap();
    let output = format!("{} {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));

    let trusted_publisher_json = load_trusted_publishers_json()?;

    let mut list_of_verify = vec![];
    for line in output.lines() {
        if line.starts_with("none ") || line.starts_with("pass ") || line.starts_with("warn ") {
            let s: Vec<&str> = line.split_whitespace().collect();
            let mut status = s[0].to_string();
            let crate_name = s[1].to_string();
            let crate_version = s[2].to_string();
            let crate_name_version = join_crate_version(&crate_name, &crate_version);

            let published_by = published_by_login(&crate_name, &crate_version)?;
            let trusted_publisher = is_trusted_publisher(&trusted_publisher_json, &published_by);
            if crate::db_yanked_mod::exists(&crate_name_version) {
                status = "yanked".to_string();
            }

            let my_review = rating_or_version(&crate_name, &crate_version)?;

            list_of_verify.push(VerifyItemData {
                status,
                my_review,
                crate_name,
                crate_version,
                published_by,
                trusted_publisher,
            })
        }
    }

    verify_sort_list_by_name_version(&mut list_of_verify);

    crate::utils_mod::ns_print_ms("verify_project", ns_started);

    Ok(VerifyListData {
        project_dir: env::current_dir()?.to_string_lossy().to_string(),
        list_of_verify,
    })
}

pub fn verify_sort_list_by_name_version(vec_of_verify: &mut Vec<VerifyItemData>) {
    vec_of_verify.sort_by(|a, b| match a.crate_name.cmp(&b.crate_name) {
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => {
            let a = semver::Version::parse(&a.crate_version).unwrap();
            let b = semver::Version::parse(&b.crate_version).unwrap();
            a.cmp(&b)
        }
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });
}
// region: cargo_crev_reviews/db_version

/// check if it is already in the cache or GET from crates.io API and store in cache
fn published_by_login(crate_name: &str, crate_version: &str) -> anyhow::Result<String> {
    let crate_name_version = &join_crate_version(crate_name, crate_version);
    let exact_version = crate::db_version_mod::read(&crate_name_version)?;
    match exact_version {
        Some(exact_version) => Ok(exact_version.published_by_login.unwrap_or("".to_string())),
        None => Ok("".to_string()),
    }
}

/// if exists my review for crate version returns the rating
/// if exists any review for this crate returns version number
/// else return empty string
fn rating_or_version(crate_name: &str, crate_version: &str) -> anyhow::Result<String> {
    let vec = crate::db_review_mod::all_versions_for_crate(crate_name)?;
    let mut version = "".to_string();
    for x in vec.iter() {
        if x.crate_version == crate_version {
            return Ok(x.rating.clone());
        }
        version = x.crate_version.clone();
    }
    Ok(version)
}
// endregion: cargo_crev_reviews/db_version

// region: cargo_crev_reviews_data/trusted_publishers.json
fn path_to_trusted_publishers_json() -> anyhow::Result<std::path::PathBuf> {
    let pb = home::home_dir()
        .context("home_dir")?
        .join(".config/crev/cargo_crev_reviews_data/trusted_publishers.json");
    Ok(pb)
}
fn load_trusted_publishers_json() -> anyhow::Result<TrustedPublisherDataFile> {
    let pb = path_to_trusted_publishers_json()?;
    if !pb.exists() {
        // first create the file empty. I will add alexcrichton, to have an example
        let trusted_publishers = vec![TrustedPublisher {
            login: "alexcrichton".to_string(),
        }];
        let trusted_publishers_data = TrustedPublisherDataFile { trusted_publishers };
        let json = serde_json::to_string_pretty(&trusted_publishers_data)?;
        std::fs::write(&pb, json)?;
    }
    let trusted_publishers_data_cache = std::fs::read_to_string(&pb)?;
    let trusted_publishers_data: TrustedPublisherDataFile = serde_json::from_str(&trusted_publishers_data_cache)?;
    Ok(trusted_publishers_data)
}
fn is_trusted_publisher(trusted_file: &TrustedPublisherDataFile, login: &str) -> String {
    for x in trusted_file.trusted_publishers.iter() {
        if &x.login == login {
            return "T".to_string();
        }
    }
    "".to_string()
}
// endregion: cargo_crev_reviews_data/trusted_publishers.json

/// get all versions for one crate
pub fn crev_crate_versions(crate_name: &str) -> anyhow::Result<Vec<VersionItemData>> {
    // region: vec_of_reviews for this crate
    let new_filter = ReviewFilterData {
        crate_name: crate_name.to_string(),
        crate_version: None,
        old_crate_version: None,
    };
    // my reviews from crev
    let vec_of_reviews = crev_list_my_reviews(&Some(new_filter))?;

    // endregion: vec_of_reviews for this crate

    // yanked from db_yanked
    let yanked_one_crate = crate::db_yanked_mod::all_versions_for_crate(crate_name)?;

    // versions from db_version
    let mut vec_of_version = vec![];
    for version_for_db in crate::db_version_mod::all_versions_for_crate(crate_name)? {
        let (_io_crate_name, io_crate_version) = split_crate_version(version_for_db.crate_name_version.as_str());

        // is_src_cache (if path exists in cargo registry src)
        let path_dir = crate::cargo_registry_mod::cargo_registry_src_dir_for_crate(crate_name, &io_crate_version)?;
        let is_src_cached = Some(path_dir.exists());

        let yanked = yanked_one_crate.iter().any(|s| s.crate_name_version == version_for_db.crate_name_version);

        // my_review
        let mut my_review = None;
        for review in vec_of_reviews.iter() {
            if review.package.name == crate_name && review.package.version.as_str() == &io_crate_version {
                my_review = Some(ReviewItemData {
                    crate_name: review.package.name.clone(),
                    crate_version: review.package.version.clone(),
                    date: review.date.clone(),
                    thoroughness: review.review.as_ref().context("review")?.thoroughness.to_string(),
                    understanding: review.review.as_ref().context("review")?.understanding.to_string(),
                    rating: rating_to_string(&review.review.as_ref().context("review")?.rating),
                    comment_md: review.comment.as_ref().unwrap_or(&String::new()).clone(),
                });
                break;
            }
        }

        // add to vector
        let version = VersionItemData {
            crate_name: crate_name.to_string(),
            crate_version: io_crate_version.to_string(),
            yanked,
            published_by_login: version_for_db.published_by_login.clone(),
            published_date: version_for_db.published_date.clone(),
            is_src_cached,
            my_review: my_review.clone(),
        };
        vec_of_version.push(version);
    }
    //write_versions_json(versions_json)?;
    Ok(vec_of_version)
}
