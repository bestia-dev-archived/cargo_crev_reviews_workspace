// response_post_method_mod.rs

//! generic functions for response to the POST method

use crate::common_structs_mod::*;
use unwrap::unwrap;

pub fn parse_post_data_and_match_method(body: &Vec<u8>) -> anyhow::Result<String> {
    let p: RpcRequest = unwrap!(serde_json::from_slice(body));
    crate::auto_generated_mod::match_request_method_and_call_function(p.request_method.as_str(), p.request_data)
}

// the first parameter is the Serialize trait and not a struct
pub fn return_srv_response<T>(response_method: &str, data: T, response_html: &str) -> String
where
    T: serde::Serialize,
{
    let data = unwrap!(serde_json::to_value(data));
    let r = RpcResponse {
        response_method: response_method.to_string(),
        response_data: data,
        response_html: response_html.to_string(),
    };
    let body = unwrap!(serde_json::to_string(&r));
    body
}

pub fn response_err_message(err: &anyhow::Error) -> anyhow::Result<String> {
    response_modal_message(err.to_string().as_str())
}

pub fn response_modal_message(msg: &str) -> anyhow::Result<String> {
    let response_data = RpcMessageData { message: msg.to_string() };
    let response_html = crate::files_mod::modal_message_html();
    crate::auto_generated_mod::cln_methods::page_review_error(response_data, response_html)
}
