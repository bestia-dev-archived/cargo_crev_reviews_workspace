// response_post_method_mod.rs

use crate::server_methods_mod::*;
use cargo_crev_reviews_common::*;
use unwrap::unwrap;

pub fn parse_post_data_and_match_method(body: &Vec<u8>) -> anyhow::Result<String> {
    let p: RpcRequest = unwrap!(serde_json::from_slice(body));
    //println!("deserialized = {:?}", &p);
    match p.request_method.as_str() {
        // here add methods that this server recognizes
        "review_list" => reviews_list_rpc(p.request_params),
        "review_save" => review_save_rpc(p.request_params),
        "review_edit" => review_edit_rpc(p.request_params),
        _ => Err(anyhow::anyhow!("unknown server method = {}", &p.request_method)),
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
