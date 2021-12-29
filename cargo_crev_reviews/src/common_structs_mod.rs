// common_structs_mod.rs

//! common structs between the backend and frontend
//! One automation task will copy it over to cargo_crev_reviews_wasm/auto_generated_mod.rs !

use serde::{Deserialize, Serialize};

// region: platform wide structs

/// the request_method will be processed on the server
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest {
    pub request_method: String,
    pub request_data: serde_json::Value,
}

/// the response_method will be processed on the client
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcResponse {
    pub response_method: String,
    pub response_data: serde_json::Value,
    pub response_html: String,
}

/// generic message for Rpc
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcMessageData {
    pub message: String,
}

/// generic empty data for Rpc
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcEmptyData {}
// endregion: platform wide structs

// region: review

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReviewFilterData {
    pub crate_name: String,
    pub crate_version: Option<String>,
    pub old_crate_version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ReviewItemData {
    pub crate_name: String,
    pub crate_version: String,
    pub date: String,
    pub thoroughness: String,
    pub understanding: String,
    pub rating: String,
    pub comment_md: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReviewListData {
    pub filter: String,
    pub list_of_review: Vec<ReviewItemData>,
}

// endregion: review

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CargoTreeItemData {
    pub cargo_tree_line: String,
    pub crate_name_version: Option<String>,
    pub my_rating: Option<String>,
    pub crate_description: Option<String>,
    pub published_by_url: Option<String>,
    pub trusted_publisher: Option<String>,
    pub status: Option<String>,
    pub audit_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CargoTreeListData {
    pub project_dir: String,
    pub list_of_cargo_tree: Vec<CargoTreeItemData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VerifyItemData {
    /// status: none, pass, warn, yanked
    pub status: String,
    /// rating if exists, version number if exists for crate
    pub my_review: String,
    pub crate_name: String,
    pub crate_version: String,
    pub published_by_url: String,
    pub trusted_publisher: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VerifyListData {
    pub project_dir: String,
    pub list_of_verify: Vec<VerifyItemData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VersionItemData {
    pub crate_name: String,
    pub crate_version: String,
    pub yanked: bool,
    pub published_by_url: Option<String>,
    pub published_date: String,
    pub is_src_cached: Option<bool>,
    pub my_review: Option<ReviewItemData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct VersionListData {
    pub list_of_version: Vec<VersionItemData>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PublisherItemData {
    pub publisher_url: String,
    pub note: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PublisherListData {
    pub list_of_publisher: Vec<PublisherItemData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PublisherFilterData {
    pub publisher_url: String,
}
