// cargo_crev_reviews_common

use serde::{Deserialize, Serialize};

// region: server - parse, match
#[derive(Serialize, Deserialize, Debug)]
pub struct RpcMethod {
    pub jsonrpc: String,
    pub method: String,
    pub params: serde_json::Value,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StringResult {
    pub jsonrpc: String,
    pub result: String,
    pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcErrorCodeMessage {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcError {
    pub jsonrpc: String,
    pub error: RpcErrorCodeMessage,
    pub id: u32,
}

// region: server - parse, match

// region: save_review
#[derive(Serialize, Deserialize, Debug)]
pub struct SaveReviewParams {
    pub crate_name: String,
    pub crate_version: String,
    pub thoroughness: String,
    pub understanding: String,
    pub rating: String,
    pub comment_md: String,
}

// endregion: save_review
