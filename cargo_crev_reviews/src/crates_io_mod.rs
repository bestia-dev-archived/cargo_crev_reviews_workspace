// crates_io_mod.rs

// communication with crates.io api
// I don't want to repeatedly use crates.io api for the same data.
// I need a disk persistent data storage.
// This crate will be used only to fill the database. Never directly from any other function.

use serde::Deserialize;
use serde::Serialize;

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
    pub num: String,
    pub yanked: bool,
    pub published_by: Option<CratesIoPublishedBy>,
    pub created_at: String,
    pub downloads: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CratesIoPublishedBy {
    pub login: String,
}

/// GET from crates.io
/// It is used only to store into db_version.
pub fn crate_response(crate_name: &str) -> anyhow::Result<CratesIoCrateResponse> {
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
    log::debug!("get url: {}", &url);
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
        let response_text = crate_responses_get("unwrap").unwrap();
        if !response_text.starts_with(r#"{"categories":[{""#) {
            panic!("wrong start");
        }
    }

    #[test]
    fn test_crate_response_deserialize() {
        let sample_text = std::fs::read_to_string("samples/crates_io_versions_for_crate.txt").unwrap();
        let _crates_io_crate_response = crate_response_deserialize(sample_text).unwrap();
    }
}
