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
pub struct RpcResult {
    pub jsonrpc: String,
    /// the name of the method that will process this response on the client. This is not in the json-rpc standard.
    pub method: String,
    pub result: serde_json::Value,
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

// endregion: server - parse, match

// region: review
#[derive(Serialize, Deserialize, Debug)]
pub struct ReviewSaveParams {
    pub crate_name: String,
    pub crate_version: String,
    pub thoroughness: String,
    pub understanding: String,
    pub rating: String,
    pub comment_md: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReviewShowParams {
    pub page_html: String,
    pub crate_name: String,
    pub crate_version: String,
    pub thoroughness: String,
    pub understanding: String,
    pub rating: String,
    pub comment_md: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ReviewEditParams {
    pub crate_name: String,
    pub crate_version: String,
    pub thoroughness: String,
    pub understanding: String,
    pub rating: String,
    pub comment_md: String,
}
// endregion: review
