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

// region: subtract
#[derive(Serialize, Deserialize, Debug)]
pub struct SubtractParams {
    pub subtrahend: f64,
    pub minuend: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SubtractResult {
    pub jsonrpc: String,
    pub result: f64,
    pub id: u32,
}

// endregion: subtract
