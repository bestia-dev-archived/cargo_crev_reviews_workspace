// crev_mod.rs

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

use crev_data::{
    proof::{CommonOps, ContentExt},
    Level, Rating,
};
use crev_lib::ProofStore;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::sync::Mutex;
use std::{ops::Range, str::FromStr, vec};
use unwrap::unwrap;

use crate::stdio_input_password_mod;

lazy_static! {
    /// mutable static, because it is hard to pass variables around with async closures
    static ref CREV_UNLOCKED: Mutex<Option<crev_data::id::UnlockedId>>=Mutex::new(None);
    static ref CREV_LOCAL: Mutex<Option<crev_lib::Local>>=Mutex::new(None);
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
    println!("Unlock the local crev id. Unlocking needs 2-3 seconds after you press Enter. Wait...");
    let crev_unlocked = crev_local.read_current_unlocked_id(&stdio_input_password_mod::read_passphrase_interactively)?;
    println!("Unlocked.");

    // write to static mut
    *CREV_UNLOCKED.lock().unwrap() = Some(crev_unlocked);
    *CREV_LOCAL.lock().unwrap() = Some(crev_local);

    // return
    Ok(())
}

/// create new review proof
pub fn create_new_review_proof(
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
    // Brute force. I didn't find functions in crate-lib, so I wrote something raw myself.
    let crate_dir_name = format!("{}-{}", crate_name, crate_version_str);
    let crate_src_path = format!(
        "{}/registry/src/github.com-1ecc6299db9ec823/{}",
        home::cargo_home()?.to_str().unwrap(),
        crate_dir_name
    );
    let crate_root = std::path::Path::new(&crate_src_path);
    if !crate_root.exists() {
        return Err(anyhow::anyhow!("The crate {}-{} does not exist in the local cargo registry cache. You must use the crate in your projects, if you want to review it. This way cargo will download the source code for the crate that you review. ", crate_name, crate_version_str));
    }
    let digest_clean = crev_lib::get_recursive_digest_for_dir(crate_root, &cargo_min_ignore_list())?;
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
    remove_review_proofs(crate_name, crate_version_str)?;

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
        _ => Err(anyhow::anyhow!("unrecognized rating: {}", rating)),
    }
}

pub fn understanding_parse(understanding: &str) -> anyhow::Result<Level> {
    match understanding.to_lowercase().as_str() {
        "none" => Ok(Level::None),
        "low" => Ok(Level::Low),
        "medium" => Ok(Level::Medium),
        "high" => Ok(Level::High),
        _ => Err(anyhow::anyhow!("unrecognized understanding: {}", understanding)),
    }
}

pub fn thoroughness_parse(thoroughness: &str) -> anyhow::Result<Level> {
    match thoroughness.to_lowercase().as_str() {
        "none" => Ok(Level::None),
        "low" => Ok(Level::Low),
        "medium" => Ok(Level::Medium),
        "high" => Ok(Level::High),
        _ => Err(anyhow::anyhow!("unrecognized thoroughness: {}", thoroughness)),
    }
}

// verify the signature of a proof
pub fn verify_proof(yaml: &str) -> anyhow::Result<()> {
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

#[derive(Deserialize, Clone, Default)]
pub struct PackageSegment {
    pub source: String,
    pub name: String,
    pub version: String,
    pub digest: String,
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
    pub package: PackageSegment,
    pub review: Option<ReviewSegment>,
}

/// remove old proofs, so the new review proof will be unique
fn remove_review_proofs(crate_name: &str, crate_version: &str) -> anyhow::Result<()> {
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

/// find and return the range of the first occurrence including start and end delimiters
/// Success: mutates also the cursor position, so the next find will continue from there
/// Fail: return None if not found and don't mutate pos_cursor
/// I use type Range to avoid references &str and lifetimes. But the programmer can make
/// the error to apply the range to the wrong vector.
pub fn find_range_including_delimiters(source_str: &str, pos_cursor: &mut usize, start_delimiter: &str, end_delimiter: &str) -> Option<std::ops::Range<usize>> {
    if let Some(pos_start) = find_pos_before_delimiter(source_str, *pos_cursor, start_delimiter) {
        // dbg!(&pos_start);
        if let Some(pos_end) = find_pos_after_delimiter(source_str, pos_start, end_delimiter) {
            // dbg!(&pos_end);
            *pos_cursor = pos_end;
            return Some(pos_start..pos_end);
        }
    }
    // return
    None
}

/// find and return the range of the first occurrence between start and end delimiters
/// Success: mutates also the cursor position, so the next find will continue from there
/// Fail: return None if not found and don't mutate pos_cursor
/// I use type Range to avoid references &str and lifetimes. But the programmer can make
/// the error to apply the range to the wrong vector.
pub fn find_range_between_delimiters(source_str: &str, pos_cursor: &mut usize, start_delimiter: &str, end_delimiter: &str) -> Option<std::ops::Range<usize>> {
    if let Some(pos_start) = find_pos_after_delimiter(source_str, *pos_cursor, start_delimiter) {
        // dbg!(&pos_start);
        if let Some(pos_end) = find_pos_before_delimiter(source_str, pos_start, end_delimiter) {
            // dbg!(&pos_end);
            *pos_cursor = pos_end + end_delimiter.len();
            return Some(pos_start..pos_end);
        }
    }
    // return
    None
}

/// return the position after the delimiter or None
/// Does NOT mutate the pos_cursor, because that is for a higher level logic to decide.
pub fn find_pos_after_delimiter(source_str: &str, pos_cursor: usize, delimiter: &str) -> Option<usize> {
    //
    if let Some(pos) = find_from(source_str, pos_cursor, delimiter) {
        let pos = pos + delimiter.len();
        return Some(pos);
    }
    // return
    None
}

/// return the position before the delimiter or None
/// Does NOT mutate the pos_cursor, because that is for a higher level logic to decide.
pub fn find_pos_before_delimiter(source_str: &str, pos_cursor: usize, delimiter: &str) -> Option<usize> {
    if let Some(pos) = find_from(source_str, pos_cursor, delimiter) {
        return Some(pos);
    }
    // return
    None
}

/// find str from pos_cursor low level
/// panics if pos_cursor is incorrect: Check for bugs in calling functions.
pub fn find_from(source_str: &str, pos_cursor: usize, find_str: &str) -> Option<usize> {
    let slice01 = unwrap!(source_str.get(pos_cursor..));
    let option_pos_found = slice01.find(find_str);
    if let Some(pos_found) = option_pos_found {
        // return Option with usize
        Some(pos_cursor + pos_found)
    } else {
        // return
        None
    }
}
