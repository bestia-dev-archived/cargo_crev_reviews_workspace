// response_post_method_mod.rs

//! generic functions for response to the POST method

use crate::common_mod::*;
use crate::rpc_methods_mod::*;
use std::str::FromStr;
use unwrap::unwrap;

pub fn parse_post_data_and_match_method(body: &Vec<u8>) -> anyhow::Result<String> {
    let p: RpcRequest = unwrap!(serde_json::from_slice(body));
    match_request_method_and_call_function(p.request_method.as_str(), p.request_data)
}

fn match_request_method_and_call_function(request_method: &str, request_data: serde_json::Value) -> anyhow::Result<String> {
    let request_enum = RequestMethod::from_str(request_method);
    match request_enum {
        Ok(request_enum) => match request_enum {
            RequestMethod::RpcReviewList => rpc_reviews_list(request_data),
            RequestMethod::RpcReviewNew => rpc_review_new(request_data),
            RequestMethod::RpcReviewSave => rpc_review_save(request_data),
            RequestMethod::RpcReviewEdit => rpc_review_edit(request_data),
            RequestMethod::RpcReviewNewVersion => rpc_review_new_version(request_data),
            RequestMethod::RpcReviewPublish => rpc_review_publish(request_data),
            RequestMethod::RpcUpdateRegistryIndex => rpc_update_registry_index(request_data),
            RequestMethod::RpcReviewOpenSourceCode => rpc_review_open_source_code(request_data),
            RequestMethod::RpcReviewDelete => rpc_review_delete(request_data),
        },
        Err(_err) => anyhow::bail!("unknown server method = {}", request_method),
    }
}

// the first parameter is the Serialize trait and not a struct
pub fn return_rpc_response<T>(response_method: ResponseMethod, data: T, response_html: &str) -> String
where
    T: serde::Serialize,
{
    let response_method: &'static str = response_method.into();
    let data = unwrap!(serde_json::to_value(data));
    let r = RpcResponse {
        response_method: response_method.to_string(),
        response_data: data,
        response_html: response_html.to_string(),
    };
    let body = unwrap!(serde_json::to_string(&r));
    body
}

pub fn response_err_message(err: &anyhow::Error) -> String {
    response_modal_message(err.to_string().as_str())
}

pub fn response_modal_message(msg: &str) -> String {
    let response_method = ResponseMethod::PageReviewError;
    let response_data = RpcMessageData { message: msg.to_string() };
    let response_html = crate::files_mod::modal_message_html();
    return_rpc_response(response_method, response_data, response_html)
}
