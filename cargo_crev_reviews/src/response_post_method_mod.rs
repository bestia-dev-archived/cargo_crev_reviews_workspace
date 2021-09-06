// response_post_method_mod.rs

use crate::response_methods_mod::*;
use cargo_crev_reviews_common::*;
use std::str::FromStr;
use unwrap::unwrap;

pub fn parse_post_data_and_match_method(body: &Vec<u8>) -> anyhow::Result<String> {
    let p: RpcRequest = unwrap!(serde_json::from_slice(body));
    match_request_method_and_call_function(p.request_method.as_str(), p.request_params)
}

fn match_request_method_and_call_function(request_method: &str, request_params: serde_json::Value) -> anyhow::Result<String> {
    let request_enum = RequestMethod::from_str(request_method);
    match request_enum {
        Ok(request_enum) => match request_enum {
            RequestMethod::ReviewList => reviews_list_rpc(request_params),
            RequestMethod::ReviewNew => review_new_rpc(request_params),
            RequestMethod::ReviewSave => review_save_rpc(request_params),
            RequestMethod::ReviewEdit => review_edit_rpc(request_params),
        },
        Err(_err) => Err(anyhow::anyhow!("unknown server method = {}",)),
    }
}

// the first parameter is the Serialize trait and not a struct
pub fn return_rpc_response<T>(client_method: &str, params: T, page_html: &str) -> String
where
    T: serde::Serialize,
{
    let params = unwrap!(serde_json::to_value(params));
    let r = RpcResponse {
        response_method: client_method.to_string(),
        response_params: params,
        page_html: page_html.to_string(),
    };
    let body = unwrap!(serde_json::to_string(&r));
    body
}
