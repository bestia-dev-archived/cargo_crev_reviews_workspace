// crates_io_mod.rs

// communication with crates.io api
// I don't want to repeatedly use crates.io api for the same data.
// I need a disk persistent data storage. I choose sled database.

use serde::Deserialize;
use serde::Serialize;

use crate::db_version_mod;
use crate::db_version_mod::VersionForDb;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CratesIoCrateResponse {
    #[serde(rename = "crate")]
    pub crate_segment: CratesIoCrate,
    pub versions: Vec<CratesIoVersion>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CratesIoCrate {
    pub name: String,
    pub max_stable_version: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CratesIoVersion {
    /// primary key
    pub crate_name_version: String,
    pub yanked: bool,
    pub published_by: Option<CratesIoPublishedBy>,
    pub created_at: String,
    pub downloads: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CratesIoPublishedBy {
    pub login: String,
}

pub fn get_version(crate_name: &str, crate_version: &str) -> anyhow::Result<Option<VersionForDb>> {
    fill_db_if_needed(crate_name)?;
    let crate_name_version = format!("{} {}", crate_name, crate_version);
    crate::db_version_mod::read(&crate_name_version)
}

/// get from cached db or GET from crates.io and stores to cache db
pub fn get_vec_of_versions(crate_name: &str) -> anyhow::Result<Vec<VersionForDb>> {
    // check if it is cached locally in the db file (sled)
    fill_db_if_needed(crate_name)?;
    let vec_version = crate::db_version_mod::all_versions_for_crate(crate_name)?;
    Ok(vec_version)
}

fn fill_db_if_needed(crate_name: &str) -> anyhow::Result<()> {
    // check if it is cached locally in the db file (sled)
    if !crate::db_version_mod::exists(crate_name) {
        let crate_resp = crate_response(crate_name)?;
        // save to cache db
        for v in crate_resp.versions.iter() {
            let published_by_login = match v.published_by.as_ref() {
                Some(published_by) => Some(published_by.login.to_string()),
                None => None,
            };
            let ver = VersionForDb {
                crate_name_version: v.crate_name_version.to_string(),
                yanked: v.yanked,
                published_by_login,
                published_date: v.created_at.to_string(),
            };
            db_version_mod::insert(&v.crate_name_version, &ver)?;
        }
    }
    Ok(())
}

fn crate_response(crate_name: &str) -> anyhow::Result<CratesIoCrateResponse> {
    let response_text = crate_responses_get(crate_name)?;
    crate_response_deserialize(response_text)
}

fn crate_response_deserialize(response_text: String) -> anyhow::Result<CratesIoCrateResponse> {
    let crates_io_crate_response: CratesIoCrateResponse = serde_json::from_str(&response_text)?;
    Ok(crates_io_crate_response)
}

fn crate_responses_get(crate_name: &str) -> Result<String, anyhow::Error> {
    // trailing slash is forbidden. See sample in file crate_io_versions_for_crate.json
    let url = format!("https://crates.io/api/v1/crates/{}", &crate_name);
    println!("get url: {}", &url);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("User-Agent", "cargo_crev_reviews (github.com/LucianoBestia/cargo_crev_reviews_workspace)")
        .send()?;
    let response_text = res.text()?;
    Ok(response_text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_responses_get() {
        let response_text = crate_responses_get("anyhow").unwrap();
        let sample_text = std::fs::read_to_string("samples/crates_io_versions_for_crate.txt").unwrap();
        assert_eq!(response_text, sample_text);
    }

    #[test]
    fn test_crate_response_deserialize() {
        let sample_text = std::fs::read_to_string("samples/crates_io_versions_for_crate.txt").unwrap();
        let _crates_io_crate_response = crate_response_deserialize(sample_text).unwrap();
    }
}
